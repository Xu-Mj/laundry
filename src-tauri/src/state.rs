use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex as TokioMutex;
use tokio::{task::JoinHandle, time::sleep};

use crate::{error::Result, local_users::LocalUser};
use crate::utils::request::{HttpClient, Token};

// SQLite 连接池
#[derive(Debug)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub http_client: HttpClient,
    pub token: Arc<TokioMutex<Option<Token>>>, // 使用 Option<Token>
    pub token_refresh_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AppState {
    pub fn new(pool: Pool<Sqlite>, base_url: impl ToString) -> Self {
        Self {
            pool,
            http_client: HttpClient::new(base_url.to_string()),
            token: Arc::new(TokioMutex::new(None)), // 初始化为 None
            token_refresh_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn refresh_token(&self) -> Result<()> {
        let mut token = self.token.lock().await;
        if let Some(token) = token.as_mut() {
            if Self::is_token_expired(&token.exp)? {
                let new_token = self.http_client.refresh_token(&token.refresh_token).await?;
                (*token).set_token(new_token);
            }
        }
        Ok(())
    }

    pub async fn start_token_refresh_task(&self) {
        let token = self.token.clone();
        let http_client = self.http_client.clone();
        let token_refresh_handle = self.token_refresh_handle.clone();

        // 检查是否已经有正在运行的 token 刷新任务
        if token_refresh_handle.lock().unwrap().is_some() {
            return;
        }

        let handle = tokio::spawn(async move {
            let mut retries = 0;
            const MAX_RETRIES: usize = 3;
            const BASE_DELAY: u64 = 2;

            loop {
                
                let mut token_ref = token.lock().await;
                if let Some(token) = token_ref.as_mut() {
                    let remaining = token.exp as i64 - SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    // 提前5分钟预刷新
                    if remaining <= 300 || Self::is_token_expired(&token.exp).unwrap_or(true) {
                        match http_client
                            .refresh_token(&token.refresh_token)
                            .await
                        {
                            Ok(new_token) => {
                                (*token).set_token(new_token);
                                retries = 0;
                                tracing::info!("Token refreshed successfully");
                            }
                            Err(e) => {
                                tracing::error!("Token refresh failed: {:?}", e);
                                if retries < MAX_RETRIES {
                                    retries += 1;
                                    let delay = BASE_DELAY.pow(retries as u32);
                                    tracing::info!("Retrying in {} seconds...", delay);
                                    sleep(Duration::from_secs(delay)).await;
                                    continue;
                                } else {
                                    tracing::error!("Max retries reached, logging out...");
                                    *token_ref = None;
                                    break;
                                }
                            }
                        }
                    }

                    // drop lock
                    drop(token_ref);
                    // 动态计算等待时间（取剩余时间和预刷新时间的较小值）
                    let wait_seconds = remaining.min(300) as u64;
                    sleep(Duration::from_secs(wait_seconds)).await;
                } else {
                    break;
                }
            }
        });

        *token_refresh_handle.lock().unwrap() = Some(handle);
    }

    fn is_token_expired(exp: &i64) -> Result<bool> {
        Ok(*exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64)
    }

    pub fn stop_token_refresh_task(&self) {
        if let Some(handle) = self.token_refresh_handle.lock().unwrap().take() {
            handle.abort();
        }
    }

    pub async fn logout(&self) {
        self.stop_token_refresh_task();
        let mut token = self.token.lock().await;
        *token = None; // 将 token 置为 None
    }
    
    pub async fn get_user_info(&self) -> Option<LocalUser> {
        let token = self.token.lock().await;
        token.as_ref().map(|t| t.user.clone())
    }

    pub async fn update_user_info(&self, user: LocalUser) {
        let mut token = self.token.lock().await;
        if let Some(token) = token.as_mut() {
            token.user = user;
        }
    }

    pub async fn get_token(&self) -> Option<String> {
        let token = self.token.lock().await;
        token.as_ref().map(|t| t.token.clone())
    }
}
