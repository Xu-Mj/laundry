use serde::{Deserialize, Serialize};
use tauri::State;

use super::DbPool;

pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// 用户ID
    pub user_id: i32,
    /// 微信标识
    pub open_id: String,
    /// 组织ID
    pub dept_id: Option<i32>,
    /// 用户账号
    pub user_name: String,
    /// 用户名称
    pub nick_name: String,
    /// 用户类型（00系统用户 01会员客户）
    pub user_type: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 手机号码
    pub phonenumber: Option<String>,
    /// 用户性别（0男 1女 2未知）
    pub sex: Option<String>,
    /// 头像地址
    pub avatar: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 帐号状态（0正常 1停用）
    pub status: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: Option<String>,
    /// 积分
    pub integral: Option<i32>,
    /// 黑灰名单（00正常 01黑名单 02灰名单）
    pub identify: Option<String>,
    /// 最后登录IP
    pub login_ip: Option<String>,
    /// 最后登录时间
    pub login_date: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 家庭住址
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
