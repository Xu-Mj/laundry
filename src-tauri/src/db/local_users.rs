use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite, Transaction};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::db::Validator;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils::device::DeviceInfo;
use crate::utils::request::Token;
use crate::{captcha, utils};

use super::sms_plan::SmsPlan;
use super::sms_subscription::SmsSubscription;
use super::subscription_plan::SubscriptionPlan;
use super::subscriptions::Subscription;
use super::user_tours::UserTours;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct LocalUser {
    pub id: Option<i64>,
    pub avatar: Option<String>,
    pub store_name: Option<String>,
    pub store_location: Option<String>,
    /// 经纪人id,引用user_id
    pub user_id: Option<i64>,
    pub owner_name: Option<String>,
    pub role: Option<String>,
    pub owner_phone: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub owner_gender: Option<String>,
    pub store_status: Option<String>,
    pub is_guest: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted: String,
    pub remark: Option<String>,
    pub nickname: Option<String>,
    /// 省份
    pub province: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// 区/县
    pub district: Option<String>,
    /// 详细地址
    pub address_detail: Option<String>,
    pub is_24_7: bool,
    /// 已完成引导的页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_tours: Option<Vec<UserTours>>,
}

impl FromRow<'_, SqliteRow> for LocalUser {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            avatar: row.try_get("avatar").unwrap_or_default(),
            store_name: row.try_get("store_name").unwrap_or_default(),
            store_location: row.try_get("store_location").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            owner_name: row.try_get("owner_name").unwrap_or_default(),
            role: row.try_get("role").unwrap_or_default(),
            owner_phone: row.try_get("owner_phone").unwrap_or_default(),
            email: row.try_get("email").unwrap_or_default(),
            password: row.try_get("password").unwrap_or_default(),
            owner_gender: row.try_get("owner_gender").unwrap_or_default(),
            store_status: row.try_get("store_status").unwrap_or_default(),
            is_guest: row.try_get("is_guest").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted: row.try_get("deleted").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            nickname: row.try_get("nickname").unwrap_or_default(),
            province: row.try_get("province").unwrap_or_default(),
            city: row.try_get("city").unwrap_or_default(),
            district: row.try_get("district").unwrap_or_default(),
            address_detail: row.try_get("address_detail").unwrap_or_default(),
            is_24_7: row.try_get("is_24_7").unwrap_or_default(),
            completed_tours: None,
        })
    }
}

impl LocalUser {
    pub fn new(username: impl ToString, password: impl ToString) -> Self {
        Self {
            owner_name: Some(username.to_string()),
            password: Some(password.to_string()),
            ..Default::default()
        }
    }
}

impl LocalUser {
    /// check account exist
    pub async fn check_username_unique(
        tr: &mut Transaction<'_, Sqlite>,
        username: &str,
    ) -> Result<bool> {
        let result =
            sqlx::query_scalar::<_, u64>("SELECT count(1) FROM local_users WHERE username = ?")
                .bind(username)
                .fetch_one(&mut **tr)
                .await?;

        Ok(result > 0)
    }

    pub async fn create(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let user = sqlx::query_as(
            "INSERT INTO local_users (
                avatar, store_name, store_location, user_id, owner_name, owner_phone, email, password, owner_gender, 
                store_status, is_guest, created_at, updated_at, deleted, remark, nickname, 
                province, city, district, address_detail, is_24_7
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
        )
        .bind(&self.avatar)
        .bind(&self.store_name)
        .bind(&self.store_location)
        .bind(self.user_id)
        .bind(&self.owner_name)
        .bind(&self.owner_phone)
        .bind(&self.email)
        .bind(&self.password)
        .bind(&self.owner_gender)
        .bind(&self.store_status)
        .bind(self.is_guest)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.deleted)
        .bind(&self.remark)
        .bind(&self.nickname)
        .bind(&self.province)
        .bind(&self.city)
        .bind(&self.district)
        .bind(&self.address_detail)
        .bind(&self.is_24_7)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Self> {
        let user: Option<LocalUser> =
            sqlx::query_as("SELECT * FROM local_users WHERE id = ? LIMIT 1")
                .bind(id)
                .fetch_optional(pool)
                .await?;

        let mut user = user.ok_or(Error::with_kind(ErrorKind::NotFound))?;

        // 获取用户的引导记录
        user.completed_tours = Some(UserTours::get_by_user_id(pool, id).await?);

        Ok(user)
    }

    pub async fn get_by_phone(pool: &Pool<Sqlite>, phone: &str) -> Result<Self> {
        let user: Option<LocalUser> =
            sqlx::query_as("SELECT * FROM local_users WHERE owner_phone = ? LIMIT 1")
                .bind(phone)
                .fetch_optional(pool)
                .await?;
        user.ok_or(Error::with_kind(ErrorKind::NotFound))
    }

    pub async fn update_pwd(pool: &Pool<Sqlite>, phone: &str, password: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE local_users SET password = ? WHERE owner_phone = ? ")
            .bind(password)
            .bind(phone)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE local_users SET
                avatar = ?,
                store_name = ?,
                store_location = ?,
                user_id = ?,
                owner_name = ?,
                owner_phone = ?,
                email = ?,
                role = ?,
                owner_gender = ?,
                store_status = ?,
                is_guest = ?,
                updated_at = ?,
                remark = ?,
                nickname = ?,
                province = ?,
                city = ?,
                district = ?,
                address_detail = ?,
                is_24_7 = ?
            WHERE id = ?",
        )
        .bind(&self.avatar)
        .bind(&self.store_name)
        .bind(&self.store_location)
        .bind(self.user_id)
        .bind(&self.owner_name)
        .bind(&self.owner_phone)
        .bind(&self.email)
        .bind(&self.role)
        .bind(&self.owner_gender)
        .bind(&self.store_status)
        .bind(self.is_guest)
        .bind(self.updated_at)
        .bind(&self.remark)
        .bind(&self.nickname)
        .bind(&self.province)
        .bind(&self.city)
        .bind(&self.district)
        .bind(&self.address_detail)
        .bind(&self.is_24_7)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn upsert(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let user:LocalUser = sqlx::query_as(
            "INSERT OR REPLACE INTO local_users (
                id, avatar, store_name, store_location, user_id, owner_name, owner_phone, email, role, password,
                owner_gender, store_status, is_guest, created_at, updated_at, deleted, remark, nickname,
                province, city, district, address_detail, is_24_7
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
        )
        .bind(self.id)
        .bind(&self.avatar)
        .bind(&self.store_name)
        .bind(&self.store_location)
        .bind(self.user_id)
        .bind(&self.owner_name)
        .bind(&self.owner_phone)
        .bind(&self.email)
        .bind(&self.role)
        .bind(&self.password)
        .bind(&self.owner_gender)
        .bind(&self.store_status)
        .bind(self.is_guest)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.deleted)
        .bind(&self.remark)
        .bind(&self.nickname)
        .bind(&self.province)
        .bind(&self.city)
        .bind(&self.district)
        .bind(&self.address_detail)
        .bind(&self.is_24_7)
        .fetch_one(&mut **pool)
        .await?;
        Ok(user)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LoginReq {
    pub account: Option<String>,
    pub password: Option<String>,
    pub code: Option<String>,
    pub uuid: Option<String>,
    // 设备信息字段
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub hardware_fingerprint: Option<String>,
}

impl Validator for LoginReq {
    fn validate(&self) -> Result<()> {
        if self.account.is_none() {
            return Err(Error::bad_request("账号不能为空"));
        }
        if self.password.is_none() {
            return Err(Error::bad_request("密码不能为空"));
        }

        Ok(())
    }
}

const PWD_SALT: &str = "xmj1234xmj1234xmj1234xmj1234xmj1234";

impl LoginReq {
    // login
    pub async fn validate_pwd(
        pool: &Pool<Sqlite>,
        username: &str,
        password: &str,
    ) -> Result<LocalUser> {
        let user: Option<LocalUser> =
            sqlx::query_as("SELECT * FROM local_users WHERE owner_phone = ? LIMIT 1")
                .bind(username)
                // .bind(&self.password)
                .fetch_optional(pool)
                .await?;

        let mut user = user.ok_or(Error::with_kind(ErrorKind::InvalidPassword))?;
        if user.password.is_none() {
            return Err(Error::internal("USER DATA ERROR"));
        }

        let parsed_hash = PasswordHash::new(user.password.as_ref().unwrap())
            .map_err(|e| Error::internal(e.to_string()))?;

        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
        user.password = Some("".to_string());
        Ok(user)
    }
}

impl LoginReq {
    #[allow(dead_code)]
    pub fn decode(&mut self) -> Result<()> {
        // base64 decode
        if self.account.is_none() || self.password.is_none() {
            return Err(Error::bad_request("parameter is none"));
        }
        let pwd = BASE64_STANDARD_NO_PAD
            .decode(self.password.as_ref().unwrap())
            .map_err(|e| Error::bad_request(e.to_string()))?;
        self.password = Some(String::from_utf8(pwd).map_err(|e| Error::internal(e.to_string()))?);
        Ok(())
    }

    async fn verify_from_server(&mut self, state: &State<'_, AppState>) -> Result<Token> {
        // 如果没有设备信息，尝试获取当前设备信息
        let device_info = crate::utils::device::DeviceInfo::get();
        self.device_id = Some(device_info.device_id);
        self.device_name = Some(device_info.device_name);
        self.device_type = Some(device_info.device_type);
        self.hardware_fingerprint = Some(device_info.hardware_fingerprint);

        tracing::debug!("登录信息: {:?}", self);

        // 发送登录请求到服务器，包含设备信息
        state.http_client.login(&self).await
    }

    async fn pull_subs_and_plans(
        state: &State<'_, AppState>,
        user_id: i64,
        token: &str,
    ) -> Result<(SubsAndPlans, SmsSubsAndPlans)> {
        let subs = state
            .http_client
            .get(&format!("/subscriptions/store/{user_id}"), Some(token))
            .await?;
        let sms_subs = state
            .http_client
            .get(&format!("/sms/subscriptions/store/{user_id}"), Some(token))
            .await?;
        Ok((subs, sms_subs))
    }

    // login, return token if success
    pub async fn login<R: Runtime>(mut self, app_handle: AppHandle<R>) -> Result<Token> {
        self.validate()?;
        let state = app_handle.state::<AppState>();

        // validate captcha
        if !captcha::verify_captcha(&state.pool, self.uuid.clone(), self.code.clone()).await? {
            return Err(Error::bad_request("验证码错误"));
        }

        // request server to validate the password
        let mut token = self.verify_from_server(&state).await?;
        // token.user.role = Some("admin".to_string());

        let mut tx = state.pool.begin().await?;
        // update local database
        token.user.upsert(&mut tx).await?;

        if let Some(id) = token.user.id {
            // get user's completed tours
            token.user.completed_tours = Some(UserTours::get_by_user_id(&state.pool, id).await?);

            let (subs, sms_subs) = Self::pull_subs_and_plans(&state, id, &token.token).await?;

            tracing::debug!("request subscriptions and plans: {:?}", subs);

            for ele in subs.subs {
                ele.upsert(&mut tx).await?;
            }
            for ele in subs.plans {
                ele.upsert(&mut tx).await?;
            }

            for ele in sms_subs.subs {
                ele.upsert(&mut tx).await?;
            }
            for ele in sms_subs.plans {
                ele.upsert(&mut tx).await?;
            }
        }
        tx.commit().await?;

        tracing::debug!("token: {:?}", token);
        // 更新 AppState 中的 token
        let mut app_token = state.token.lock().await;
        *app_token = Some(token.clone());
        drop(app_token);
        // 启动 token 刷新任务
        state.start_jobs(app_handle.clone()).await?;

        Ok(token)
    }
}

#[derive(Debug, Deserialize)]
pub struct SubsAndPlans {
    pub subs: Vec<Subscription>,
    pub plans: Vec<SubscriptionPlan>,
}

#[derive(Debug, Deserialize)]
pub struct SmsSubsAndPlans {
    pub subs: Vec<SmsSubscription>,
    pub plans: Vec<SmsPlan>,
}

// Guest login logical
async fn guest_login_logical(state: &State<'_, AppState>) -> Result<Token> {
    // select by id
    let user = LocalUser::get_by_id(&state.pool, 0).await?;
    let token = Token {
        user,
        exp: 60 * 60 * 12,
        token: "guest token".into(),
        refresh_token: "guest refresh token".into(),
    };
    // 更新 AppState 中的 token
    let mut app_token = state.token.lock().await;
    *app_token = Some(token.clone());
    Ok(token)
}

#[tauri::command]
pub async fn login<R: Runtime>(app_handle: AppHandle<R>, req: LoginReq) -> Result<Token> {
    req.login(app_handle).await
}

#[tauri::command]
pub async fn guest_login(mut state: State<'_, AppState>) -> Result<Token> {
    guest_login_logical(&mut state).await
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<()> {
    state.logout().await;
    Ok(())
}

#[tauri::command]
pub async fn validate_pwd(state: State<'_, AppState>, account: &str, pwd: &str) -> Result<()> {
    LoginReq::validate_pwd(&state.pool, account, pwd).await?;
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: LocalUser,
    pub subscriptions: Vec<Subscription>, // 新增字段，存储所有有效订阅
    pub sms_subscriptions: Vec<SmsSubscription>, // 存储所有有效短信订阅
}

#[tauri::command]
pub async fn get_info(state: State<'_, AppState>) -> Result<UserInfo> {
    let store_id = utils::get_user_id(&state).await?;
    let user = LocalUser::get_by_id(&state.pool, store_id).await?;
    let mut subscriptions = Vec::new();
    let mut sms_subscriptions = Vec::new();

    if let Some(id) = user.id {
        if id > 0 {
            // 获取所有有效订阅
            subscriptions = Subscription::get_all_active_by_user_id(&state.pool, id).await?;

            // 获取所有有效短信订阅
            sms_subscriptions = SmsSubscription::get_all_active_by_user_id(&state.pool, id).await?;
        }
    }

    Ok(UserInfo {
        user,
        subscriptions,
        sms_subscriptions,
    })
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePwdReq {
    pub account: String,
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePwdReqForServer {
    pub id: i64,
    pub password: String,
}

#[tauri::command]
pub async fn update_pwd(state: State<'_, AppState>, req: UpdatePwdReq) -> Result<bool> {
    // validate
    if req.account.is_empty() {
        return Err(Error::bad_request("账户错误"));
    } else if req.new_password.len() < 5 || req.new_password.len() > 20 {
        return Err(Error::bad_request("密码长度必须在5到20个字符之间"));
    } else if req.new_password != req.confirm_password {
        return Err(Error::bad_request("两次输入的密码不一致"));
    }

    let pool = &state.pool;
    // validate old password
    LoginReq::validate_pwd(pool, &req.account, &req.old_password).await?;

    let password = utils::hash_password(req.new_password.as_bytes(), PWD_SALT)?;

    // update password in server

    let user = LocalUser::get_by_phone(pool, &req.account).await?;

    tracing::debug!(
        "update_pwd: user={:?} token: {:?}",
        user,
        state.get_token().await
    );
    // get token from state
    let _user: LocalUser = state
        .http_client
        .put(
            "/stores/pwd",
            UpdatePwdReqForServer {
                id: user.id.unwrap(),
                password: password.clone(),
            },
            state.get_token().await.as_deref(),
        )
        .await?;

    tracing::debug!("update_pwd33333: user={:?}", _user);
    LocalUser::update_pwd(pool, &req.account, &password).await
}

#[tauri::command]
pub async fn update_local_user(state: State<'_, AppState>, user: LocalUser) -> Result<LocalUser> {
    // 验证用户信息
    if user.id.is_none() {
        return Err(Error::bad_request("用户ID不能为空"));
    }

    // 设置更新时间
    let mut updated_user = user.clone();
    updated_user.updated_at = utils::get_timestamp();

    // 先向服务端发送更新请求
    tracing::debug!("Updating user on server: {:?}", updated_user);

    let server_user: LocalUser = state
        .http_client
        .put("/stores", &updated_user, state.get_token().await.as_deref())
        .await?;

    // 服务端更新成功后，更新本地数据库
    tracing::debug!("Server update successful, updating local database");
    let success = server_user.update(&state.pool).await?;

    if !success {
        return Err(Error::internal("更新本地数据库失败"));
    }

    // 返回更新后的用户信息
    Ok(server_user)
}

// 获取当前设备信息并验证
#[tauri::command]
pub async fn get_device_info() -> Result<DeviceInfo> {
    Ok(DeviceInfo::get())
}

// register
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreRegisterVerifyRequest {
    pub phone: String,
}

// 商户注册验证码响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreRegisterVerifyResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn get_sms_verification_code(
    state: State<'_, AppState>,
    req: StoreRegisterVerifyRequest,
) -> Result<StoreRegisterVerifyResponse> {
    // send request to server
    let res = state.http_client.post("/register/code", &req, None).await?;
    Ok(res)
}
// 商户注册响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreRegisterResponse {
    pub success: bool,
    pub message: String,
    pub store_id: Option<i64>,
}

// 商户注册请求
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RegisterRequest {
    pub avatar: String,
    pub name: String,
    pub nickname: String,
    pub location: String,
    // 新增地址字段
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address_detail: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
    pub owner_gender: Option<String>,
    pub password: String,
    pub email: String,
    pub verification_code: String,
    pub remark: Option<String>,
    // 设备信息
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub mac_address: Option<String>,
    pub hardware_fingerprint: Option<String>,
}

#[tauri::command]
pub async fn register(
    state: State<'_, AppState>,
    req: RegisterRequest,
) -> Result<StoreRegisterResponse> {
    // send request to server
    let res = state
        .http_client
        .post("/register/store", &req, None)
        .await?;
    Ok(res)
}
