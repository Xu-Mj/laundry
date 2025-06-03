use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime, State};

use crate::db::order_pictures::OrderPicture;
use crate::error::Error;
use crate::error::{ErrorKind, Result};
use crate::state::AppState;

#[derive(Debug, Default, Serialize)]
pub struct ImageUploadResult {
    pub id: i64,
    pub path: String,
}

impl ImageUploadResult {
    pub fn new(id: i64, path: String) -> Self {
        Self { id, path }
    }
}

#[tauri::command]
pub async fn save_image<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, AppState>,
    name: String,
    data: Vec<u8>,
) -> Result<ImageUploadResult> {
    // 获取 PathResolver
    let path_resolver = app_handle.path();

    // 获取应用的本地数据目录
    let app_local_data_dir = path_resolver.app_data_dir().map_err(|e| {
        Error::with_details(
            ErrorKind::InternalServer,
            format!("无法获取应用的本地数据目录: {:?}", e),
        )
    })?;

    let app_local_data_dir = app_local_data_dir.join("images");
    // 如果目录不存在，则创建
    if !app_local_data_dir.exists() {
        std::fs::create_dir_all(&app_local_data_dir)?;
    }

    // 构建保存路径
    let save_path = app_local_data_dir.join(name);

    let path = save_path.to_string_lossy().to_string();
    // 使用事物写入数据
    let mut tx = state.pool.begin().await?;
    let picture = OrderPicture::new_with_path(&path).insert(&mut tx).await?;
    // 将数据写入文件
    let mut file = File::create(&path)?;
    file.write_all(&data)?;

    tx.commit().await?;
    // 返回保存路径
    Ok(ImageUploadResult::new(picture.picture_id.unwrap(), path))
}

#[tauri::command]
pub async fn delete_image(state: State<'_, AppState>, id: i64) -> Result<()> {
    let pool = &state.pool;
    let picture = OrderPicture::get_by_id(pool, id)
        .await?
        .ok_or(Error::not_found("图片未找到或已被删除"))?;
    let mut tx = pool.begin().await?;
    OrderPicture::delete(&mut tx, id).await?;
    // 删除文件
    if let Some(path) = picture.picture_path {
        std::fs::remove_file(&path)?;
    }
    tx.commit().await?;
    Ok(())
}

#[tauri::command]
pub async fn get_image(state: State<'_, AppState>, id: i64) -> Result<Vec<u8>> {
    let picture = OrderPicture::get_by_id(&state.pool, id)
        .await?
        .ok_or(Error::not_found("图片未找到或已被删除"))?;

    if picture.picture_path.is_none() {
        return Err(Error::not_found("图片未找到或已被删除"));
    }
    // 读取图片文件
    let image_data = std::fs::read(&picture.picture_path.unwrap())?;

    // 返回图片的二进制数据
    Ok(image_data)
}

#[tauri::command]
pub async fn save_file<R: Runtime>(
    app_handle: AppHandle<R>,
    name: String,
    data: Vec<u8>,
) -> Result<String> {
    // 获取 PathResolver
    let path_resolver = app_handle.path();

    // 获取应用的本地数据目录
    let app_local_data_dir = path_resolver.app_data_dir().map_err(|e| {
        Error::with_details(
            ErrorKind::InternalServer,
            format!("无法获取应用的本地数据目录: {:?}", e),
        )
    })?;

    // 创建密钥文件目录
    let key_files_dir = app_local_data_dir.join("files");
    if !key_files_dir.exists() {
        std::fs::create_dir_all(&key_files_dir)?;
    }

    // 构建保存路径
    let save_path = key_files_dir.join(name);

    // 将数据写入文件
    let mut file = File::create(&save_path)?;
    file.write_all(&data)?;

    // 返回保存路径
    let path = save_path.to_string_lossy().to_string();
    Ok(path)
}

#[tauri::command]
pub async fn delete_file(file_path: String) -> Result<()> {
    // 检查文件路径是否存在
    let path = Path::new(&file_path);
    if path.exists() {
        // 删除文件
        std::fs::remove_file(path)?;
    }
    Ok(())
}
