use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::{Error, ErrorKind, Result};
use crate::local_users::LocalUser;
use crate::state::AppState;

const URL_LOGIN: &str = "/stores/login";
const URL_REFRESH_TOKEN: &str = "/stores/refresh-token";

pub trait Request: Serialize + DeserializeOwned + Send + Sized {
    const URL: &'static str;
    async fn create_request(&self, state: &State<'_, AppState>) -> Result<Self> {
        let result = state
            .http_client
            .post(Self::URL, self, Some(&state.try_get_token().await?))
            .await?;
        Ok(result)
    }
    async fn update_request(&self, state: &State<'_, AppState>) -> Result<bool> {
        let result = state
            .http_client
            .put(Self::URL, self, Some(&state.try_get_token().await?))
            .await?;
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        HttpClient { client, base_url }
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T> {
        let response = request.send().await?;
        if response.status().is_success() {
            let res = response.json::<T>().await?;
            Ok(res)
        } else {
            let status = response.status();
            let error: Error = response.json().await.unwrap_or(Error::with_details(
                ErrorKind::ReqwestError,
                format!("Request failed with status: {}", status),
            ));
            Err(error)
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

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
        let url = format!(
            "{}{URL_REFRESH_TOKEN}/{}/true",
            self.base_url, refresh_token
        );
        let request = self.client.get(&url);
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
    pub token: String,
    pub refresh_token: String,
    pub exp: i64,
}

impl Token {
    pub fn set_token(&mut self, token: String) {
        self.token = token;
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
