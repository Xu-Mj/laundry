pub(crate) mod chrono_serde;

use std::collections::HashMap;

use alibaba_cloud_sdk_rust::services::dysmsapi;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use pinyin::ToPinyin;
use rand::distributions::Uniform;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use sqlx::types::chrono::{DateTime, FixedOffset, Utc};

use crate::error::{Error, Result};

pub fn gen_code(input: impl ToString) -> String {
    let input = input.to_string().trim().to_uppercase();
    if input.is_empty() {
        return "".to_string();
    }

    let mut result = String::new();

    for c in input.chars() {
        if is_chinese(c) {
            if let Some(py) = c.to_pinyin() {
                result.push_str(
                    &py.plain()
                        .chars()
                        .next()
                        .unwrap_or(' ')
                        .to_string()
                        .to_uppercase(),
                );
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// gen by chatgpt
#[inline]
fn is_chinese(c: char) -> bool {
    (c >= '\u{4E00}' && c <= '\u{9FFF}')
        || (c >= '\u{3400}' && c <= '\u{4DBF}') // CJK Extension A
        || (c >= '\u{20000}' && c <= '\u{2A6DF}') // CJK Extension B
        || (c >= '\u{2A700}' && c <= '\u{2B73F}') // CJK Extension C
        || (c >= '\u{2B740}' && c <= '\u{2B81F}') // CJK Extension D
        || (c >= '\u{2B820}' && c <= '\u{2CEAF}') // CJK Extension E
        || (c >= '\u{2CEB0}' && c <= '\u{2EBEF}') // CJK Extension F
        || (c >= '\u{F900}' && c <= '\u{FAFF}') // CJK Compatibility Ideographs
        || (c >= '\u{2F800}' && c <= '\u{2FA1F}') // CJK Compatibility Ideographs Supplement
}
const EAST_ZONE: i32 = 8 * 60 * 60;

pub fn get_now() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(EAST_ZONE).unwrap())
}

pub fn gen_random_number() -> i32 {
    // 使用随机种子初始化 StdRng
    let seed = rand::random::<[u8; 32]>();
    let mut rng = StdRng::from_seed(seed);

    // 生成一个六位随机数
    let range = Uniform::from(100000..=999999);
    let random_number: i32 = rng.sample(&range);

    random_number
}

/// 将给定的值转换为布尔值。
///
/// 该函数接受一个实现了ToString trait的泛型参数T，将其转换为字符串，
/// 并判断其内容是否匹配布尔值true的常见字符串表示之一。
/// 匹配时，返回true；否则，返回false。
///
/// # 参数
/// - `value`: 实现了ToString trait的任何类型，表示要转换为布尔值的输入。
///
/// # 返回值
/// - `bool`: 根据输入值是否匹配布尔值true的字符串表示，返回相应的布尔值。
pub fn to_bool<T: ToString>(value: T) -> bool {
    // 将输入值转换为字符串，去除前后空格，转为小写，并进行匹配
    matches!(
        value.to_string().trim().to_lowercase().as_str(),
        "true" | "yes" | "ok" | "1"
    )
}

pub fn hash_password(password: &[u8], salt: &str) -> Result<String> {
    // 使用默认的Argon2配置
    // 这个配置可以更改为适合您具体安全需求和性能要求的设置

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password, &SaltString::from_b64(salt).unwrap())
        .map_err(|e| Error::internal(e.to_string()))?
        .to_string())
}

const ALIYUN_SMS_SERVER_REGION: &str = "cn-hangzhou";
const ALIYUN_SMS_ACCESS_KEY_ID: &str = "LTAI5tGbR8DaKsV7ivxH1gGm";
const ALIYUN_SMS_ACCESS_KEY_SECRET: &str = "bCWn3BBRE2YRAWPhZr471cWou73zJi";
const ALIYUN_SMS_REPORT_TEMPLATE_CODE: &str = " SMS_474905744"; // 通知模版
const ALIYUN_SMS_SIGN_NAME: &str = "沈阳市浑南区印洗世家洗护 "; // 短信署名

pub fn send_sms(phone_number: &str, parameters: Option<HashMap<String, String>>) -> Result<bool> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        ALIYUN_SMS_SERVER_REGION,
        ALIYUN_SMS_ACCESS_KEY_ID,
        ALIYUN_SMS_ACCESS_KEY_SECRET,
    )?;
    let mut request = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = phone_number.replace("+86", "");
    request.SignName = ALIYUN_SMS_SIGN_NAME.to_owned();
    request.TemplateCode = ALIYUN_SMS_REPORT_TEMPLATE_CODE.to_owned();
    if let Some(parameters) = parameters {
        request.TemplateParam = serde_json::to_string(&parameters)?;
    }

    let response = client.SendSms(&mut request)?;
    if response.Code.contains("OK") {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utils::{gen_code, is_chinese, send_sms};

    #[test]
    fn test_get_initial() {
        let input = "hello你好";
        let initials = gen_code(input);
        assert_eq!(initials, "HELLONH");
    }

    #[test]
    fn test_is_chinese() {
        let input = 'h';
        let initials = is_chinese(input);
        assert!(initials);
    }
    #[test]
    fn send_sms_test() {
        let params = HashMap::from([("code".to_string(), "123456".to_string())]);
        send_sms("+8617863935638", Some(params)).unwrap();
    }
}
