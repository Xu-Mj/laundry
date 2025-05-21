use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Emitter, Runtime};
use tokio::sync::Mutex as TokioMutex;
use tokio::{task::JoinHandle, time::sleep};

use crate::{
    error::Error,
    orders::TimeWarningManager,
    utils::request::{HttpClient, Token},
};
use crate::{error::Result, local_users::LocalUser};

// Token 刷新配置
const MIN_REFRESH_INTERVAL: u64 = 60; // 最小刷新间隔（秒）
const MAX_REFRESH_INTERVAL: u64 = 3600; // 最大刷新间隔（秒）
const REFRESH_THRESHOLD: u64 = 300; // 提前刷新阈值（秒）
const MAX_RETRIES: usize = 3; // 最大重试次数
const INITIAL_RETRY_DELAY: u64 = 2; // 初始重试延迟（秒）
const SLEEP_REFRESH_DELAY: u64 = 5; // 系统唤醒后延迟刷新时间（秒）

// SQLite 连接池
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub http_client: HttpClient,
    pub token: Arc<TokioMutex<Option<Token>>>,
    pub token_refresh_handle: Arc<TokioMutex<Option<JoinHandle<()>>>>,
    pub time_warning_check_handle: TimeWarningManager,
    pub last_activity_time: Arc<Mutex<SystemTime>>,
}

impl AppState {
    pub fn new(pool: Pool<Sqlite>, base_url: impl ToString) -> Self {
        Self {
            pool,
            http_client: HttpClient::new(base_url.to_string()),
            token: Arc::new(TokioMutex::new(None)),
            token_refresh_handle: Arc::new(TokioMutex::new(None)),
            time_warning_check_handle: TimeWarningManager::new(),
            last_activity_time: Arc::new(Mutex::new(SystemTime::now())),
        }
    }

    // 更新最后活动时间
    pub fn update_activity_time(&self) {
        *self.last_activity_time.lock().unwrap() = SystemTime::now();
    }

    // 检查是否需要立即刷新 token（系统唤醒后）
    pub async fn check_and_refresh_after_sleep(&self) {
        let token_lock = self.token.lock().await;
        if let Some(token) = token_lock.as_ref() {
            if token.user.id == Some(0) {
                tracing::debug!("logined user is guest, no need to refresh");
                return;
            }

            // 检查 token 是否过期或即将过期
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let remaining = token.exp - now;

            // 如果 token 已过期或即将过期（小于阈值），则刷新
            if remaining <= REFRESH_THRESHOLD as i64 {
                tracing::info!("Token is expired or about to expire, refreshing...");

                // 等待一小段时间，确保网络连接已恢复
                sleep(Duration::from_secs(SLEEP_REFRESH_DELAY)).await;

                // 释放锁，避免死锁
                drop(token_lock);

                // 强制刷新 token
                if let Err(e) = self.refresh_token().await {
                    tracing::error!("Failed to refresh token after sleep: {:?}", e);
                }
            } else {
                tracing::debug!("Token is still valid, no need to refresh");
            }
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

    pub async fn start_jobs<R: Runtime>(&self, app_handle: AppHandle<R>) -> Result<()> {
        tracing::info!("start jobs");
        self.start_token_refresh_task(app_handle.clone()).await;
        self.time_warning_check_handle
            .start(app_handle.clone())
            .await?;
        Ok(())
    }

    pub async fn start_token_refresh_task<R: Runtime>(&self, app_handle: AppHandle<R>) {
        let token = self.token.clone();
        let http_client = self.http_client.clone();
        let token_refresh_handle = self.token_refresh_handle.clone();

        // 检查是否已经有正在运行的 token 刷新任务
        if token_refresh_handle.lock().await.is_some() {
            return;
        }

        let handle = tokio::spawn(async move {
            let mut retries = 0;
            let mut last_refresh_time = SystemTime::now();

            loop {
                let mut token_ref = token.lock().await;
                if let Some(token) = token_ref.as_mut() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    let remaining = token.exp - now;
                    let time_since_last_refresh = now
                        - last_refresh_time
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64;

                    // 计算下一次刷新时间
                    let next_refresh = if remaining <= REFRESH_THRESHOLD as i64 {
                        // 如果剩余时间小于阈值，立即刷新
                        0
                    } else {
                        // 否则，使用动态计算的时间
                        let dynamic_interval = (remaining - REFRESH_THRESHOLD as i64) / 2;
                        dynamic_interval
                            .max(MIN_REFRESH_INTERVAL as i64)
                            .min(MAX_REFRESH_INTERVAL as i64)
                    };

                    // 检查是否需要刷新
                    if time_since_last_refresh >= next_refresh {
                        match http_client.refresh_token(&token.refresh_token).await {
                            Ok(new_token) => {
                                (*token).set_token(new_token);
                                last_refresh_time = SystemTime::now();
                                retries = 0;
                                tracing::info!("Token refreshed successfully");
                            }
                            Err(e) => {
                                tracing::error!("Token refresh failed: {:?}", e);
                                if retries < MAX_RETRIES {
                                    retries += 1;
                                    let delay = INITIAL_RETRY_DELAY * (1 << (retries - 1));
                                    tracing::info!("Retrying in {} seconds...", delay);
                                    drop(token_ref); // 释放锁
                                    sleep(Duration::from_secs(delay)).await;
                                    continue;
                                } else {
                                    tracing::error!("Max retries reached, logging out...");
                                    *token_ref = None;
                                    // 发送登出事件
                                    let result = app_handle.emit("app://logout", ());
                                    if let Err(e) = result {
                                        tracing::error!("Failed to emit logout event: {:?}", e);
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    // 释放锁
                    drop(token_ref);

                    // 计算等待时间
                    let wait_seconds = next_refresh - time_since_last_refresh;
                    if wait_seconds > 0 {
                        sleep(Duration::from_secs(wait_seconds as u64)).await;
                    }
                } else {
                    break;
                }
            }
        });

        *token_refresh_handle.lock().await = Some(handle);
    }

    fn is_token_expired(exp: &i64) -> Result<bool> {
        Ok(*exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64)
    }

    pub async fn stop_token_refresh_task(&self) {
        if let Some(handle) = self.token_refresh_handle.lock().await.take() {
            handle.abort();
        }
    }

    pub async fn logout(&self) {
        self.stop_token_refresh_task().await;
        // 清除 token
        let mut token = self.token.lock().await;
        *token = None; // 将 token 置为 None
        self.time_warning_check_handle.stop().await;
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

    pub async fn try_get_token(&self) -> Result<String> {
        let token = self.token.lock().await;
        Ok(token.as_ref().ok_or(Error::unauthorized())?.token.clone())
    }

    pub async fn try_token(&self) -> Result<Token> {
        let token = self.token.lock().await;
        Ok(token.clone().ok_or(Error::unauthorized())?)
    }

    pub async fn try_get_user_id(&self) -> Result<i64> {
        let token = self.token.lock().await;
        Ok(token
            .as_ref()
            .ok_or(Error::unauthorized())?
            .user
            .id
            .ok_or(Error::unauthorized())?)
    }
}
