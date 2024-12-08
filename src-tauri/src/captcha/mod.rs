use base64::prelude::*;
use chrono::{Duration, Utc};
use image::{ImageBuffer, Luma};
use once_cell::sync::Lazy;
use rand::Rng;
use rusttype::{point, Font, Scale};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration as StdDuration;
use uuid::Uuid;

use crate::db::configs::Config;
use crate::db::AppState;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaResp {
    pub captcha_enabled: bool,
    pub uuid: String,
    pub img: String,
}

impl CaptchaResp {
    pub fn disabled_captcha() -> Self {
        Self {
            captcha_enabled: false,
            uuid: "".to_string(),
            img: "".to_string(),
        }
    }

    pub fn with_img(uuid: String, img: String) -> Self {
        Self {
            captcha_enabled: true,
            uuid,
            img,
        }
    }
}

// 全局线程安全的验证码存储
pub static CAPTCHA_STORE: Lazy<Mutex<HashMap<String, (String, chrono::DateTime<Utc>)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// 生成验证码的命令
#[tauri::command]
pub async fn get_captcha(state: tauri::State<'_, AppState>) -> Result<CaptchaResp> {
    let pool = &state.0;

    if !Config::is_captcha_enabled(pool).await? {
        return Ok(CaptchaResp::disabled_captcha());
    }

    // 生成 UUID 作为验证码唯一标识
    let uuid = Uuid::new_v4().to_string();

    // query config
    let config = Config::get_config_by_key(pool, "captcha_type").await?;

    let value = config
        .and_then(|c| c.config_value)
        .unwrap_or("math".to_string());
    // 根据类型生成验证码
    let (code, image) = match value.as_str() {
        "math" => {
            // 生成简单数学题
            let a: i32 = rand::thread_rng().gen_range(10..99);
            let b: i32 = rand::thread_rng().gen_range(1..9);
            let result = a + b;
            let captcha_str = format!("{} + {} =", a, b);
            (result.to_string(), generate_captcha_image(&captcha_str))
        }
        "char" => {
            // 生成随机字符验证码
            let code: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();
            let image = generate_captcha_image(&code);
            (code, image)
        }
        _ => return Err(Error::internal("Invalid captcha type")),
    };

    // 将验证码存储到内存中，设置5分钟过期时间
    let expiration_time = Utc::now() + Duration::minutes(5);
    CAPTCHA_STORE
        .lock()
        .map_err(|_| Error::internal("Failed to acquire lock on CAPTCHA_STORE"))?
        .insert(uuid.clone(), (code, expiration_time));

    // 编码图像为 Base64
    let mut buffer = std::io::Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, image::ImageFormat::Jpeg)
        .map_err(|e| Error::internal(e.to_string()))?;
    let img_base64 = BASE64_STANDARD.encode(buffer.into_inner());

    // 返回响应

    Ok(CaptchaResp::with_img(uuid, img_base64))
}

// 验证验证码的命令
pub async fn verify_captcha(
    pool: &Pool<Sqlite>,
    uuid: Option<String>,
    input_code: Option<String>,
) -> Result<bool> {
    if !Config::is_captcha_enabled(pool).await? {
        return Ok(true);
    }
    tracing::debug!("uuid: {:?}, input_code: {:?}", uuid, input_code);
    if uuid.is_none() || input_code.is_none() {
        return Err(Error::bad_request("Parameter is none"));
    }
    let (uuid, input_code) = (uuid.unwrap(), input_code.unwrap());

    let mut store = CAPTCHA_STORE
        .lock()
        .map_err(|_| Error::internal("Failed to acquire lock on CAPTCHA_STORE"))?;

    if let Some((code, expiration)) = store.get(&uuid) {
        if Utc::now() > *expiration {
            store.remove(&uuid); // 移除过期验证码
            Err(Error::internal("Captcha expired"))
        } else if *code.to_lowercase() == input_code.to_lowercase() {
            store.remove(&uuid); // 验证成功后移除验证码
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err(Error::not_found("Captcha not found"))
    }
}

// 生成验证码图像
fn generate_captcha_image(text: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = 200;
    let height = 50;
    let mut img = ImageBuffer::from_pixel(width, height, Luma([255u8]));
    let mut rng = rand::thread_rng();

    // 添加随机噪点
    for pixel in img.pixels_mut() {
        if rng.gen_range(0..100) > 90 {
            *pixel = Luma([rng.gen_range(0..255) as u8]);
        }
    }

    // 加载字体
    let font_data = include_bytes!("../../MSYH.TTC"); // 替换为实际字体路径
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // 设置字体大小
    let scale = Scale::uniform(30.0); // 字体大小

    // 确定文本的起始位置
    let v_metrics = font.v_metrics(scale);
    let start = point(10.0, 10.0 + v_metrics.ascent); // 偏移文本到适合的起点

    // 绘制文本
    for glyph in font.layout(text, scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as u32 + bb.min.x as u32;
                let y = y as u32 + bb.min.y as u32;

                if x < width && y < height {
                    let pixel = img.get_pixel_mut(x, y);
                    let intensity = (1.0 - v) * 255.0; // 将文本颜色设置为灰度
                    *pixel = Luma([intensity as u8]);
                }
            });
        }
    }
    img
}

// 启动清理线程
pub fn start_cleanup_thread() {
    thread::spawn(|| loop {
        // 每分钟清理一次
        thread::sleep(StdDuration::from_secs(60));

        // 清理过期的验证码
        if let Ok(mut store) = CAPTCHA_STORE.lock() {
            let now = Utc::now();
            store.retain(|_, (_, expiration)| *expiration > now);
        }
    });
}
