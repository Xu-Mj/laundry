use std::collections::HashSet;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite, Transaction};
use tauri::State;

use crate::db::Validator;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils::request::Token;
use crate::{captcha, utils};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
    pub first_login: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted: String,
    pub remark: Option<String>,
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
            first_login: row.try_get("first_login").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted: row.try_get("deleted").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
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
                avatar, store_name, store_location, user_id, owner_name, owner_phone, email, password, owner_gender, store_status, first_login, created_at, updated_at, deleted, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
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
        .bind(self.first_login)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.deleted)
        .bind(&self.remark)
        .fetch_one(pool)
        .await?;
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

    pub async fn update_first_login(pool: &Pool<Sqlite>, account: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE local_users SET first_login = 0 WHERE owner_phone = ? ")
            .bind(account)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn upsert(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let user = sqlx::query_as(
            "INSERT OR REPLACE INTO local_users (
                id, avatar, store_name, store_location, user_id, owner_name, owner_phone, email, role, password,
                 owner_gender, store_status, first_login, created_at, updated_at, deleted, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
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
        .bind(self.first_login)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.deleted)
        .bind(&self.remark)
        .fetch_one(pool)
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

    async fn verify_from_server(&self, state: &State<'_, AppState>) -> Result<Token> {
        state.http_client.login(self).await
    }

    // login, return token if success
    pub async fn login(self, state: &State<'_, AppState>) -> Result<Token> {
        self.validate()?;

        // validate captcha
        if !captcha::verify_captcha(&state.pool, self.uuid.clone(), self.code.clone()).await? {
            return Err(Error::bad_request("验证码错误"));
        }

        // request server to validate the password
        let mut token = self.verify_from_server(state).await?;
        token.user.role = Some("admin".to_string());

        // update local database
        token.user.upsert(&state.pool).await?;

        tracing::debug!("token: {:?}", token);
        // 更新 AppState 中的 token
        let mut app_token = state.token.lock().await;
        *app_token = Some(token.clone());

        // 启动 token 刷新任务
        state.start_token_refresh_task().await;

        Ok(token)
    }

    // pub async fn register(mut self, pool: &Pool<Sqlite>) -> Result<()> {
    //     self.decode()?;

    //     let mut tr = pool.begin().await?;

    //     let username = self.username.unwrap_or_default();
    //     let password = self.password.unwrap_or_default();

    //     // validate captcha
    //     // if !captcha::verify_captcha(&mut tr, self.uuid.clone(), self.code.clone()).await? {
    //     //     return Err(Error::bad_request("验证码错误"));
    //     // }

    //     if username.is_empty() {
    //         return Err(Error::bad_request("用户名不能为空"));
    //     } else if password.is_empty() {
    //         return Err(Error::bad_request("用户密码不能为空"));
    //     } else if username.len() < 2 || username.len() > 20 {
    //         return Err(Error::bad_request("账户长度必须在2到20个字符之间"));
    //     } else if password.len() < 5 || password.len() > 20 {
    //         return Err(Error::bad_request("密码长度必须在5到20个字符之间"));
    //     } else if LocalUser::check_username_unique(&mut tr, &username).await? {
    //         return Err(Error::bad_request(format!(
    //             "保存用户'{}'失败，注册账号已存在",
    //             username
    //         )));
    //     } else {
    //         // encode the password
    //         let password = utils::hash_password(password.as_bytes(), PWD_SALT)?;
    //         let mut user = LocalUser::new(username.clone(), password);
    //         user.avatar = Some(String::from("images/avatar1.png"));
    //         user.owner_phone = Some(username);
    //         user.role = Some(String::from("admin"));
    //         user.create(pool).await?;
    //     }

    //     Ok(())
    // }
}

#[tauri::command]
pub async fn login(mut state: State<'_, AppState>, req: LoginReq) -> Result<Token> {
    req.login(&mut state).await
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<()> {
    state.logout().await;
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: Option<LocalUser>,
    pub roles: HashSet<String>,
    pub permissions: HashSet<String>,
}

#[tauri::command]
pub async fn get_info(state: State<'_, AppState>) -> Result<UserInfo> {
    let user = state.get_user_info().await;
    Ok(UserInfo {
        user,
        roles: HashSet::from(["admin".to_string()]),
        permissions: HashSet::from(["*:*:*".to_string()]),
    })
}

// #[tauri::command]
// pub async fn register(state: State<'_, AppState>, req: LoginReq) -> Result<()> {
//     req.register(&state.pool).await
// }

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

    tracing::debug!("update_pwd: user={:?} token: {:?}", user,state.get_token().await);
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
    LocalUser::update_pwd(pool, &req.account, &password).await?;
    // update first_login
    LocalUser::update_first_login(pool, &req.account).await
}
