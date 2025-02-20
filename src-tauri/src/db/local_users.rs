use crate::db::{AppState, Validator};
use crate::error::{Error, ErrorKind, Result};
use crate::{captcha, utils};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite, Transaction};
use std::collections::HashSet;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct LocalUser {
    pub id: i64,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub remark: Option<String>,
    pub is_first_login: bool,
}

impl FromRow<'_, SqliteRow> for LocalUser {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            username: row.try_get("username").unwrap_or_default(),
            avatar: row.try_get("avatar").unwrap_or_default(),
            account: row.try_get("account").unwrap_or_default(),
            password: row.try_get("password").unwrap_or_default(),
            role: row.try_get("role").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            is_first_login: row.try_get("is_first_login").unwrap_or_default(),
        })
    }
}

impl LocalUser {
    pub fn new(username: impl ToString, password: impl ToString) -> Self {
        Self {
            username: Some(username.to_string()),
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
        let user = sqlx::query_as::<_, LocalUser>(
            "INSERT INTO local_users (username, avatar, account, password, role, remark) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
        )
            .bind(&self.username)
            .bind(&self.avatar)
            .bind(&self.account)
            .bind(&self.password)
            .bind(&self.role)
            .bind(&self.remark)
            .fetch_one(pool).await?;
        Ok(user)
    }

    pub async fn update_pwd(pool: &Pool<Sqlite>, account: &str, password: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE local_users SET password = ? WHERE account = ? ")
            .bind(password)
            .bind(account)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_is_first_login(pool: &Pool<Sqlite>, account: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE local_users SET is_first_login = 0 WHERE account = ? ")
            .bind(account)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Token {
    pub user: LocalUser,
    pub token: String,
}

pub const REFRESH_EXPIRES: i64 = 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

const EXPIRES: i64 = 60 * 60 * 4;

impl Claims {
    pub fn new(sub: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        let exp = now + EXPIRES;
        Self { sub, exp, iat: now }
    }
}

impl LoginReq {
    // login
    pub async fn validate_pwd(
        pool: &Pool<Sqlite>,
        username: &str,
        password: &str,
    ) -> Result<LocalUser> {
        let user: Option<LocalUser> =
            sqlx::query_as("SELECT * FROM local_users WHERE account = ? LIMIT 1")
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LoginReq {
    pub username: Option<String>,
    pub password: Option<String>,
    pub code: Option<String>,
    pub uuid: Option<String>,
}

impl Validator for LoginReq {
    fn validate(&self) -> Result<()> {
        if self.username.is_none() {
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
    pub fn decode(&mut self) -> Result<()> {
        // base64 decode
        if self.username.is_none() || self.password.is_none() {
            return Err(Error::bad_request("parameter is none"));
        }
        let pwd = BASE64_STANDARD_NO_PAD
            .decode(self.password.as_ref().unwrap())
            .map_err(|e| Error::bad_request(e.to_string()))?;
        self.password = Some(String::from_utf8(pwd).map_err(|e| Error::internal(e.to_string()))?);
        Ok(())
    }

    // login, return token if success
    pub async fn login(mut self, pool: &Pool<Sqlite>) -> Result<Token> {
        self.validate()?;

        // validate captcha
        if !captcha::verify_captcha(pool, self.uuid.clone(), self.code.clone()).await? {
            return Err(Error::bad_request("验证码错误"));
        }

        // decode password
        self.decode()?;

        let user = Self::validate_pwd(
            pool,
            self.username.as_ref().unwrap(),
            self.password.as_ref().unwrap(),
        )
        .await?;

        // generate token
        let mut claims = Claims::new(user.username.clone().unwrap());

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("app_state.jwt_secret.as_bytes()".as_bytes()),
        )?;

        claims.exp += REFRESH_EXPIRES;
        Ok(Token { user, token })
    }

    pub async fn register(mut self, pool: &Pool<Sqlite>) -> Result<()> {
        self.decode()?;

        let mut tr = pool.begin().await?;

        let username = self.username.unwrap_or_default();
        let password = self.password.unwrap_or_default();

        // validate captcha
        // if !captcha::verify_captcha(&mut tr, self.uuid.clone(), self.code.clone()).await? {
        //     return Err(Error::bad_request("验证码错误"));
        // }

        if username.is_empty() {
            return Err(Error::bad_request("用户名不能为空"));
        } else if password.is_empty() {
            return Err(Error::bad_request("用户密码不能为空"));
        } else if username.len() < 2 || username.len() > 20 {
            return Err(Error::bad_request("账户长度必须在2到20个字符之间"));
        } else if password.len() < 5 || password.len() > 20 {
            return Err(Error::bad_request("密码长度必须在5到20个字符之间"));
        } else if LocalUser::check_username_unique(&mut tr, &username).await? {
            return Err(Error::bad_request(format!(
                "保存用户'{}'失败，注册账号已存在",
                username
            )));
        } else {
            // encode the password
            let password = utils::hash_password(password.as_bytes(), PWD_SALT)?;
            let mut user = LocalUser::new(username.clone(), password);
            user.avatar = Some(String::from("images/avatar1.png"));
            user.account = Some(username);
            user.role = Some(String::from("admin"));
            user.create(pool).await?;
        }

        Ok(())
    }
}

pub static LOGIN_USER: Lazy<Mutex<Option<LocalUser>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub async fn login(state: State<'_, AppState>, req: LoginReq) -> Result<Token> {
    let token = req.login(&state.pool).await?;
    {
        *LOGIN_USER.lock().unwrap() = Some(token.user.clone());
    }

    state.update_last_login_time();

    Ok(token)
}

#[tauri::command]
pub async fn logout() -> Result<()> {
    {
        *LOGIN_USER.lock().unwrap() = None;
    }

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: Option<LocalUser>,
    pub roles: HashSet<String>,
    pub permissions: HashSet<String>,
}

#[tauri::command]
pub async fn get_info() -> Result<UserInfo> {
    let user = LOGIN_USER.lock().unwrap().clone();
    Ok(UserInfo {
        user,
        roles: HashSet::from(["admin".to_string()]),
        permissions: HashSet::from(["*:*:*".to_string()]),
    })
}

#[tauri::command]
pub async fn register(state: State<'_, AppState>, req: LoginReq) -> Result<()> {
    req.register(&state.pool).await
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePwdReq {
    pub account: String,
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
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
    LocalUser::update_pwd(&state.pool, &req.account, &password).await?;
    // update is_first_login
    LocalUser::update_is_first_login(&state.pool, &req.account).await
}

#[cfg(test)]
mod tests {
    use super::*;
    // test register
    #[tokio::test]
    async fn test_register() {
        let password = BASE64_STANDARD_NO_PAD.encode("xmj20241025");

        let req = LoginReq {
            username: Some("admin1".to_string()),
            password: Some(password),
            code: Some("1234".to_string()),
            uuid: Some("1234".to_string()),
        };
        let pool = sqlx::SqlitePool::connect("sqlite://database.db")
            .await
            .unwrap();
        req.register(&pool).await.unwrap();
    }
}
