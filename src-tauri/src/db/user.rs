use serde::{Deserialize, Serialize};
use tauri::State;

use super::DbPool;

pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: i32,
    /// 微信标识
    #[serde(rename = "openId")]
    pub open_id: String,
    /// 组织ID
    #[serde(rename = "deptId", skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i32>,
    /// 用户账号
    #[serde(rename = "userName")]
    pub user_name: String,
    /// 用户名称
    #[serde(rename = "nickName")]
    pub nick_name: String,
    /// 用户类型（00系统用户 01会员客户）
    #[serde(rename = "userType", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// 用户邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号码
    #[serde(rename = "phonenumber", skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<String>,
    /// 用户性别（0男 1女 2未知）
    #[serde(rename = "sex", skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 头像地址
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 密码
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 帐号状态（0正常 1停用）
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    #[serde(rename = "delFlag", skip_serializing_if = "Option::is_none")]
    pub del_flag: Option<String>,
    /// 积分
    #[serde(rename = "integral", skip_serializing_if = "Option::is_none")]
    pub integral: Option<i32>,
    /// 黑灰名单（00正常 01黑名单 02灰名单）
    #[serde(rename = "identify", skip_serializing_if = "Option::is_none")]
    pub identify: Option<String>,
    /// 最后登录IP
    #[serde(rename = "loginIp", skip_serializing_if = "Option::is_none")]
    pub login_ip: Option<String>,
    /// 最后登录时间
    #[serde(rename = "loginDate", skip_serializing_if = "Option::is_none")]
    pub login_date: Option<String>,
    /// 创建者
    #[serde(rename = "createBy", skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新者
    #[serde(rename = "updateBy", skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 备注
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 家庭住址
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}


// 插入数据的命令
#[tauri::command]
pub async fn insert_user(state: State<'_, DbPool>, name: String) -> Result<(), String> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(name)
        .execute(&state.0)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// 查询数据的命令
#[tauri::command]
pub async fn get_users(state: State<'_, DbPool>) -> Result<Vec<User>, String> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&state.0)
        .await
        .map_err(|e| e.to_string())?;
    Ok(users)
}
