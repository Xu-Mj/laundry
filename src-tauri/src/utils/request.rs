use log::{error, info, warn};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tauri::State;
use tokio::time::sleep;

use crate::error::{Error, ErrorKind, Result};
use crate::local_users::LocalUser;
use crate::state::AppState;

const URL_LOGIN: &str = "/stores/login";
const URL_REFRESH_TOKEN: &str = "/stores/refresh_token";
const DEFAULT_TIMEOUT: u64 = 30; // 默认超时时间（秒）
const MAX_RETRIES: u32 = 3; // 最大重试次数
const RATE_LIMIT_PER_SECOND: u32 = 10; // 每秒最大请求数

#[derive(Debug, Deserialize, Serialize)]
pub struct StoreIdWithIds {
    pub store_id: i64,
    pub ids: Vec<i64>,
}

pub trait Request: Serialize + DeserializeOwned + Send + Sized + Default {
    const URL: &'static str;
    async fn create_request(&self, state: &State<'_, AppState>) -> Result<Self> {
        let token = state.try_token().await?;
        if token.user.id == Some(0) {
            return Ok(Self::default());
        }

        let result = state
            .http_client
            .post(Self::URL, self, Some(&token.token))
            .await?;
        Ok(result)
    }
    async fn update_request(&self, state: &State<'_, AppState>) -> Result<bool> {
        let token = state.try_token().await?;
        if token.user.id == Some(0) {
            return Ok(true);
        }
        let result = state
            .http_client
            .put(Self::URL, self, Some(&token.token))
            .await?;
        Ok(result)
    }
    async fn delete_request(state: &State<'_, AppState>, body: StoreIdWithIds) -> Result<bool> {
        let token = state.try_token().await?;
        if token.user.id == Some(0) {
            return Ok(true);
        }

        let result = state
            .http_client
            .delete(Self::URL, body, Some(&token.token))
            .await?;
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    request_count: Arc<AtomicU32>,
    last_reset: Arc<std::sync::atomic::AtomicU64>,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()
            .unwrap();
        HttpClient {
            client,
            base_url,
            request_count: Arc::new(AtomicU32::new(0)),
            last_reset: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    async fn check_rate_limit(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let last_reset = self.last_reset.load(Ordering::Relaxed);
        if now - last_reset >= 1 {
            self.request_count.store(0, Ordering::Relaxed);
            self.last_reset.store(now, Ordering::Relaxed);
        }

        while self.request_count.load(Ordering::Relaxed) >= RATE_LIMIT_PER_SECOND {
            sleep(Duration::from_millis(100)).await;
        }

        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T> {
        self.check_rate_limit().await;

        // 检查是否可以克隆来决定是否支持重试
        if let Some(cloned) = request.try_clone() {
            // 支持重试的逻辑
            let mut retries = 0;
            let mut current_request = cloned;

            loop {
                match current_request.send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let url = response.url().clone();
                            match response.json::<T>().await {
                                Ok(res) => {
                                    info!("Request successful: {}", url);
                                    return Ok(res);
                                }
                                Err(e) => {
                                    error!("Failed to parse response: {}", e);
                                    return Err(Error::with_details(
                                        ErrorKind::ReqwestError,
                                        format!("Failed to parse response: {}", e),
                                    ));
                                }
                            }
                        } else {
                            let status = response.status();

                            // 处理 401 未授权错误
                            if status == reqwest::StatusCode::UNAUTHORIZED {
                                warn!("Received 401 Unauthorized response");
                                return Err(Error::with_details(
                                    ErrorKind::UnAuthorized,
                                    "Authentication required".to_string(),
                                ));
                            }

                            let error: Error =
                                response.json().await.unwrap_or(Error::with_details(
                                    ErrorKind::ReqwestError,
                                    format!("Request failed with status: {}", status),
                                ));

                            if status.is_server_error() && retries < MAX_RETRIES {
                                retries += 1;
                                warn!("Retrying request (attempt {}/{})", retries, MAX_RETRIES);
                                current_request = request.try_clone().unwrap();
                                continue;
                            }

                            error!("Request failed: {:?}", error);
                            return Err(error);
                        }
                    }
                    Err(e) => {
                        if retries < MAX_RETRIES {
                            retries += 1;
                            warn!(
                                "Retrying request (attempt {}/{}): {}",
                                retries, MAX_RETRIES, e
                            );
                            current_request = request.try_clone().unwrap();
                            continue;
                        }
                        error!("Request failed after {} retries: {}", MAX_RETRIES, e);
                        return Err(Error::with_details(
                            ErrorKind::ReqwestError,
                            format!("Request failed: {}", e),
                        ));
                    }
                }
            }
        } else {
            // 不支持重试，直接发送一次
            warn!("Request body is not cloneable, sending without retry support");
            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let url = response.url().clone();
                        match response.json::<T>().await {
                            Ok(res) => {
                                info!("Request successful: {}", url);
                                return Ok(res);
                            }
                            Err(e) => {
                                error!("Failed to parse response: {}", e);
                                return Err(Error::with_details(
                                    ErrorKind::ReqwestError,
                                    format!("Failed to parse response: {}", e),
                                ));
                            }
                        }
                    } else {
                        let status = response.status();

                        // 处理 401 未授权错误
                        if status == reqwest::StatusCode::UNAUTHORIZED {
                            warn!("Received 401 Unauthorized response");
                            return Err(Error::with_details(
                                ErrorKind::UnAuthorized,
                                "Authentication required".to_string(),
                            ));
                        }

                        let error: Error = response.json().await.unwrap_or(Error::with_details(
                            ErrorKind::ReqwestError,
                            format!("Request failed with status: {}", status),
                        ));

                        error!("Request failed (no retry): {:?}", error);
                        return Err(error);
                    }
                }
                Err(e) => {
                    error!("Request failed (no retry): {}", e);
                    return Err(Error::with_details(
                        ErrorKind::ReqwestError,
                        format!("Request failed: {}", e),
                    ));
                }
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str, token: Option<&str>) -> Result<T> {
        let mut request = self.client.get(&format!("{}{}", self.base_url, endpoint));
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }
        self.send_request(request).await
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: B,
        token: Option<&str>,
    ) -> Result<T> {
        let mut request = self.client.post(&format!("{}{}", self.base_url, endpoint));
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }
        self.send_request(request.json(&body)).await
    }

    pub async fn login<T: DeserializeOwned, B: Serialize>(&self, body: B) -> Result<T> {
        let url = format!("{}{URL_LOGIN}", self.base_url);
        let request = self.client.post(&url).json(&body);
        self.send_request(request).await
    }

    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: B,
        token: Option<&str>,
    ) -> Result<T> {
        let mut request = self.client.put(&format!("{}{}", self.base_url, endpoint));
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }
        self.send_request(request.json(&body)).await
    }

    pub async fn delete<B: Serialize>(
        &self,
        endpoint: &str,
        body: B,
        token: Option<&str>,
    ) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.delete(&url);
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }
        self.send_request(request.json(&body)).await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &str,
        is_refresh: bool,
    ) -> Result<TokenResponse> {
        let url = format!("{}{}", self.base_url, URL_REFRESH_TOKEN);
        let req = RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
            is_refresh,
        };
        let request = self.client.post(&url).json(&req);
        self.send_request(request).await
    }

    /// 上传单个文件
    ///
    /// # 参数
    /// * `endpoint` - API端点
    /// * `file_path` - 本地文件路径
    /// * `field_name` - 表单字段名称，默认为"file"
    /// * `token` - 认证令牌
    pub async fn upload_file<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        file_path: &str,
        field_name: &str,
        token: Option<&str>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);

        // 检查文件是否存在
        if !std::path::Path::new(file_path).exists() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                format!("文件不存在: {}", file_path),
            ));
        }

        // 检查文件大小
        let file_size = std::fs::metadata(file_path)
            .map_err(|e| {
                Error::with_details(ErrorKind::BadRequest, format!("无法获取文件大小: {}", e))
            })?
            .len();

        // 设置最大文件大小限制（例如 10MB）
        const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
        if file_size > MAX_FILE_SIZE {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                format!("文件大小超过限制: {} bytes", file_size),
            ));
        }

        // 读取文件内容
        let file_data = std::fs::read(file_path).map_err(|e| {
            Error::with_details(
                ErrorKind::BadRequest,
                format!("无法读取文件 {}: {}", file_path, e),
            )
        })?;

        // 获取文件名
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown.file");

        // 创建multipart表单
        let part = reqwest::multipart::Part::bytes(file_data).file_name(file_name.to_string());

        let form = reqwest::multipart::Form::new().part(field_name.to_string(), part);

        // 构建请求
        let mut request = self.client.post(&url).multipart(form);
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }

        info!("Uploading file: {} to {}", file_path, url);
        self.send_request(request).await
    }

    /// 批量上传多个文件
    ///
    /// # 参数
    /// * `endpoint` - API端点
    /// * `file_paths` - 本地文件路径列表
    /// * `field_name` - 表单字段名称，默认为"file"
    /// * `token` - 认证令牌
    pub async fn upload_files<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        file_paths: &[String],
        field_name: &str,
        token: Option<&str>,
    ) -> Result<T> {
        if file_paths.is_empty() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "没有提供文件路径",
            ));
        }

        let url = format!("{}{}", self.base_url, endpoint);

        // 创建multipart表单
        let mut form = reqwest::multipart::Form::new();

        // 添加每个文件到表单
        for path in file_paths {
            // 读取文件内容
            let file_data = std::fs::read(path).map_err(|e| {
                Error::with_details(
                    ErrorKind::BadRequest,
                    format!("无法读取文件 {}: {}", path, e),
                )
            })?;

            // 获取文件名
            let file_name = std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.file");

            // 创建文件部分并添加到表单
            let part = reqwest::multipart::Part::bytes(file_data).file_name(file_name.to_string());

            form = form.part(field_name.to_string(), part);
        }

        // 构建请求
        let mut request = self.client.post(&url).multipart(form);
        if let Some(t) = token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }

        self.send_request(request).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
    is_refresh: bool,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    token: String,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub code: u8,
    pub data: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Token {
    pub user: LocalUser,
    pub token: String, // access_token
    pub refresh_token: String,
    pub access_exp: i64,  // access_token 过期时间
    pub refresh_exp: i64, // refresh_token 过期时间（新增）
}
impl Token {
    pub fn set_token(&mut self, token: TokenResponse) {
        self.token = token.token;
        self.access_exp = token.exp;
    }
}

#[cfg(test)]
mod tests {
    use crate::notice_temp::NoticeTemp;

    use super::HttpClient;

    #[tokio::test]
    async fn test_token_error() {
        let client = HttpClient::new(String::from("http://localhost:8080"));
        let rows: Vec<NoticeTemp> = match client.get("/msg_temps", None).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                Vec::new()
            }
        };
        println!("{:?}", rows);
    }
}
