use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind, Result};
use crate::local_users::LocalUser;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        HttpClient {
            client: Client::new(),
            base_url,
        }
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
        let url = format!("{}/stores/login", self.base_url);
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

    pub async fn delete(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, endpoint);
        let request = self.client.delete(&url);
        self.send_request(request).await
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
        let url = format!("{}/refresh-token/{}/true", self.base_url, refresh_token);
        let request = self.client.get(&url);
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
