use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;

use crate::constants::CouponType;
use crate::db::order_clothes::OrderCloth;
use crate::db::orders::Order;
use crate::db::user_coupons::UserCoupon;
use crate::db::user_membership_level::UserMembershipLevel;
use crate::db::user_tags::UserTags;
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::Request;

use super::{Curd, PageParams, PageResult};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct User {
    /// 所属商店ID
    pub store_id: Option<i64>,
    /// 用户ID
    pub user_id: Option<i64>,
    /// 微信标识
    pub open_id: Option<String>,
    /// 组织ID
    pub dept_id: Option<i32>,
    /// 用户账号
    pub user_name: Option<String>,
    /// 用户名称
    pub nick_name: Option<String>,
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
    #[serde(skip_serializing)]
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
    pub create_time: Option<i64>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 备注
    pub remark: Option<String>,
    /// 家庭住址
    pub address: Option<String>,

    /// tags
    pub user_tags: Option<String>,
    pub tags_remark: Option<String>,

    /// 余额
    pub balance: f64,

    /// 等级名称
    pub level_id: Option<i64>,
    pub level_name: Option<String>,
}

impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(User {
            store_id: row.try_get("store_id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            open_id: row.try_get("open_id").unwrap_or_default(),
            dept_id: row.try_get("dept_id").unwrap_or_default(),
            user_name: row.try_get("user_name").unwrap_or_default(),
            nick_name: row.try_get("nick_name").unwrap_or_default(),
            user_type: row.try_get("user_type").unwrap_or_default(),
            email: row.try_get("email").unwrap_or_default(),
            phonenumber: row.try_get("phonenumber").unwrap_or_default(),
            sex: row.try_get("sex").unwrap_or_default(),
            avatar: row.try_get("avatar").unwrap_or_default(),
            password: row.try_get("password").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            integral: row.try_get("integral").unwrap_or_default(),
            identify: row.try_get("identify").unwrap_or_default(),
            login_ip: row.try_get("login_ip").unwrap_or_default(),
            login_date: row.try_get("login_date").unwrap_or_default(),
            create_by: row.try_get("create_by").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_by: row.try_get("update_by").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            address: row.try_get("address").unwrap_or_default(),
            user_tags: row.try_get("user_tags").unwrap_or_default(),
            tags_remark: row.try_get("tags_remark").unwrap_or_default(),
            level_name: row.try_get("level_name").unwrap_or_default(),
            level_id: row.try_get("level_id").unwrap_or_default(),
            balance: 0.,
        })
    }
}

impl User {
    #[inline]
    fn init(&mut self) {
        if self.integral.is_none() {
            self.integral = Some(0);
        }

        if self.identify.is_none() {
            self.identify = Some("00".to_string());
        }

        if self.user_type.is_none() {
            self.user_type = Some("01".to_string());
        }

        if self.status.is_none() {
            self.status = Some("0".to_string());
        }

        if self.del_flag.is_none() {
            self.del_flag = Some("0".to_string());
        }
    }
}
const USER_MEMBERSHIP_COSTUMER: i64 = 3;

const QUERY_SQL: &str = "select u.*, p.level_name, p.level_id, t.tags AS user_tags,
        t.remark as tags_remark from users u
        left join user_membership_level up on u.user_id = up.user_id
        left join membership_level p on up.level_id = p.level_id
        left join user_tags t  on t.user_id = u.user_id
        where u.del_flag = '0'";

const BY_ID_SQL: &str = "select u.*, p.level_name, p.level_id, t.tags AS user_tags,
        t.remark as tags_remark from users u
        left join user_membership_level up on u.user_id = up.user_id
        left join membership_level p on up.level_id = p.level_id
        left join user_tags t  on t.user_id = u.user_id
        where u.del_flag = '0' AND u.user_id = ? ";

impl Curd for User {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM users u LEFT JOIN user_membership_level up ON u.user_id = up.user_id WHERE u.del_flag = '0' ";
    const QUERY_SQL: &'static str = QUERY_SQL;
    const BY_ID_SQL: &'static str = BY_ID_SQL;
    const DELETE_BATCH_SQL: &'static str = "UPDATE users SET del_flag = '2' WHERE user_id IN ( ";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        self.user_name.as_ref().filter(|u| !u.is_empty()).map(|u| {
            builder
                .push(" AND u.user_name LIKE ")
                .push_bind(format!("%{}%", u));
        });
        self.nick_name.as_ref().filter(|u| !u.is_empty()).map(|u| {
            builder
                .push(" AND u.nick_name LIKE ")
                .push_bind(format!("%{}%", u));
        });

        self.phonenumber
            .as_ref()
            .filter(|p| !p.is_empty())
            .map(|p| {
                builder
                    .push(" AND u.phonenumber LIKE ")
                    .push_bind(format!("%{}%", p));
            });

        self.level_id.filter(|l| *l != 0).map(|l| {
            builder.push(" AND up.level_id = ").push_bind(l);
        });

        self.store_id.filter(|l| *l >= 0).map(|l| {
            builder.push(" AND u.store_id = ").push_bind(l);
        });

        self.status.as_ref().filter(|s| !s.is_empty()).map(|s| {
            builder.push(" AND u.status = ").push_bind(s);
        });
    }
}

impl User {
    pub async fn create(&self, tr: &mut sqlx::Transaction<'_, Sqlite>) -> Result<Self> {
        let row = sqlx::query_as(
            "INSERT INTO users (
            user_id, store_id, open_id, dept_id, user_name, nick_name, user_type, email, phonenumber,
            sex, avatar, password, status, integral, identify, login_ip,
            login_date, create_by, create_time, remark, address, del_flag
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '0')
        RETURNING *",
        )
        .bind(&self.user_id)
        .bind(&self.store_id)
        .bind(&self.open_id)
        .bind(&self.dept_id)
        .bind(&self.user_name)
        .bind(&self.nick_name)
        .bind(&self.user_type)
        .bind(&self.email)
        .bind(&self.phonenumber)
        .bind(&self.sex)
        .bind(&self.avatar)
        .bind(&self.password)
        .bind(&self.status)
        .bind(&self.integral)
        .bind(&self.identify)
        .bind(&self.login_ip)
        .bind(&self.login_date)
        .bind(&self.create_by)
        .bind(utils::get_timestamp())
        .bind(&self.remark)
        .bind(&self.address)
        .fetch_one(&mut **tr)
        .await?;
        Ok(row)
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE users SET 
            open_id = ?, 
            dept_id = ?, 
            user_name = ?, 
            nick_name = ?, 
            user_type = ?, 
            email = ?, 
            phonenumber = ?, 
            sex = ?, 
            avatar = ?, 
            password = ?, 
            status = ?, 
            integral = ?, 
            identify = ?, 
            login_ip = ?, 
            login_date = ?, 
            update_by = ?, 
            remark = ?, 
            address = ?, 
            update_time = ? 
            WHERE store_id = ? AND user_id = ?",
        )
        .bind(&self.open_id)
        .bind(&self.dept_id)
        .bind(&self.user_name)
        .bind(&self.nick_name)
        .bind(&self.user_type)
        .bind(&self.email)
        .bind(&self.phonenumber)
        .bind(&self.sex)
        .bind(&self.avatar)
        .bind(&self.password)
        .bind(&self.status)
        .bind(&self.integral)
        .bind(&self.identify)
        .bind(&self.login_ip)
        .bind(&self.login_date)
        .bind(&self.update_by)
        .bind(&self.remark)
        .bind(&self.address)
        .bind(utils::get_timestamp())
        .bind(self.store_id)
        .bind(self.user_id)
        .execute(&mut **tr)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // list by ids
    pub async fn list_by_ids(pool: &Pool<Sqlite>, ids: &[i64]) -> Result<Vec<Self>> {
        let mut query = QueryBuilder::new("SELECT * FROM users WHERE user_id IN ( ");

        ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                query.push(", ");
            }
            query.push_bind(id);
        });

        query.push(" )");

        let result = query.build_query_as().fetch_all(pool).await?;
        Ok(result)
    }

    /// check account exist
    pub async fn check_user_name_unique(
        pool: &Pool<Sqlite>,
        store_id: i64,
        user_name: &str,
    ) -> Result<bool> {
        let result = sqlx::query_scalar::<_, u64>(
            "SELECT count(1) FROM users WHERE del_flag = '0' AND store_id = ? AND user_name = ?",
        )
        .bind(store_id)
        .bind(user_name)
        .fetch_one(pool)
        .await?;

        Ok(result > 0)
    }

    pub async fn check_tel_unique(pool: &Pool<Sqlite>, store_id: i64, tel: &str) -> Result<bool> {
        let result = sqlx::query_scalar::<_, u64>(
            "SELECT count(1) FROM users WHERE del_flag = '0' AND store_id = ? AND phonenumber = ?",
        )
        .bind(store_id)
        .bind(tel)
        .fetch_one(pool)
        .await?;

        Ok(result > 0)
    }

    /// increase user points
    pub async fn increase_points(
        tx: &mut Transaction<'_, Sqlite>,
        id: i64,
        points: i64,
    ) -> Result<bool> {
        let result = sqlx::query("UPDATE users SET integral = integral + ? WHERE user_id = ?")
            .bind(points)
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// decrease user points
    pub async fn decrease_points(
        tx: &mut Transaction<'_, Sqlite>,
        id: i64,
        points: i64,
    ) -> Result<bool> {
        let result =
            sqlx::query("UPDATE users SET integral = MAX(integral - ?, 0) WHERE user_id = ?")
                .bind(points)
                .bind(id)
                .execute(&mut **tx)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    /// update address
    pub async fn update_address(
        tx: &mut Transaction<'_, Sqlite>,
        store_id: i64,
        id: i64,
        address: &str,
    ) -> Result<bool> {
        let result = sqlx::query("UPDATE users SET address =? WHERE store_id = ? AND user_id =?")
            .bind(address)
            .bind(store_id)
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

/// Service
impl User {
    pub async fn get_user_by_cloth_code(
        pool: &Pool<Sqlite>,
        store_id: i64,
        cloth_code: &str,
    ) -> Result<Option<Self>> {
        let cloth = OrderCloth::get_by_cloth_code(pool, cloth_code)
            .await?
            .ok_or(Error::not_found("cloth not found"))?;

        if cloth.order_id.is_none() {
            return Err(Error::internal("cloth data error"));
        }

        let order = Order::get_by_id(pool, store_id, cloth.order_id.unwrap_or_default())
            .await?
            .ok_or(Error::not_found("order not found"))?;

        Self::get_by_id(pool, order.user_id.unwrap_or_default()).await
    }

    pub async fn create_user(&mut self, state: &State<'_, AppState>) -> Result<Self> {
        let pool = &state.pool;
        let mut tr = pool.begin().await?;

        // validate
        if self.store_id.is_none() {
            return Err(Error::unauthorized());
        }

        if self.phonenumber.is_none() {
            return Err(Error::bad_request("手机号不能为空"));
        } else if User::check_tel_unique(
            pool,
            self.store_id.unwrap(),
            self.phonenumber.as_ref().unwrap(),
        )
        .await?
        {
            return Err(Error::bad_request("手机号已经存在"));
        }

        if self.user_name.is_none() {
            self.user_name = self.phonenumber.clone();
        }

        if User::check_user_name_unique(
            pool,
            self.store_id.unwrap(),
            self.user_name.as_ref().unwrap(),
        )
        .await?
        {
            return Err(Error::bad_request("账号已存在"));
        }

        self.init();

        let user_tags = self.user_tags.clone();
        let tags_remark = self.tags_remark.clone();
        // create user to server
        let user = self.create_request(state).await?;

        let user = user.create(&mut tr).await?;
        // create user tags
        if let Some(tags) = user_tags {
            UserTags::new(user.user_id.unwrap(), tags, tags_remark)
                .insert(&mut tr)
                .await?;
        }

        // combine member level
        UserMembershipLevel::new(user.user_id.unwrap(), USER_MEMBERSHIP_COSTUMER)
            .create(&mut tr)
            .await?;
        tr.commit().await?;
        Ok(user)
    }

    pub async fn update_user(&self, state: &State<'_, AppState>) -> Result<bool> {
        let pool = &state.pool;
        let mut tr = pool.begin().await?;
        // validate
        if self.store_id.is_none() {
            return Err(Error::unauthorized());
        }

        if self.user_id.is_none() {
            return Err(Error::bad_request("user id can not be empty"));
        }

        // update user tags
        if let Some(tags) = &self.user_tags {
            UserTags::new(
                self.user_id.unwrap(),
                tags.clone(),
                self.tags_remark.clone(),
            )
            .update(&mut tr)
            .await?;
        }

        // udpate user to server
        if !self.update_request(state).await? {
            return Err(Error::bad_request("update user to server failed"));
        }

        // update user to database
        let result = self.update(&mut tr).await?;
        tr.commit().await?;
        Ok(result)
    }

    pub async fn pagination(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Self>> {
        let mut result = self.get_list(pool, page_params).await?;
        for user in result.rows.iter_mut() {
            // query user coupons for storage card and calculate balance
            if let Some(id) = user.user_id {
                let coupons = UserCoupon::find_by_user_id(pool, self.store_id.unwrap(), id).await?;
                user.balance = coupons
                    .iter()
                    .filter(|c| {
                        if let Some(coupon) = c.coupon.as_ref() {
                            if let Some(valid_to) = coupon.valid_to {
                                return coupon.coupon_type == Some(CouponType::StoredValueCard)
                                    && utils::get_now() <= valid_to
                                    && c.available_value.is_some()
                                    && c.available_value.unwrap() > 0.;
                            }
                        }
                        false
                    })
                    .map(|c| c.available_value.unwrap_or_default())
                    .sum();
            }
        }
        Ok(result)
    }

    pub async fn delete_users(pool: &Pool<Sqlite>, ids: Vec<i64>) -> Result<bool> {
        let mut tr = pool.begin().await?;
        if !User::delete_batch(&mut tr, &ids).await? {
            return Err(Error::internal("delete user failed"));
        }

        // delete user coupons
        // if !UserCoupon::delete_by_user_ids(&mut tr, &ids).await? {
        //     return Err(Error::internal("delete user coupons failed"));
        // }

        tr.commit().await?;
        Ok(true)
    }
}

impl Request for User {
    const URL: &'static str = "/users";
}

#[tauri::command]
pub async fn get_users_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    mut user: User,
) -> Result<PageResult<User>> {
    let store_id = utils::get_user_id(&state).await?;
    user.store_id = Some(store_id);
    user.pagination(&state.pool, page_params).await
}

#[tauri::command]
pub async fn get_all_users(state: State<'_, AppState>, mut user: User) -> Result<Vec<User>> {
    let store_id = utils::get_user_id(&state).await?;
    user.store_id = Some(store_id);
    user.status = Some("0".to_string());
    user.get_all(&state.pool).await
}

#[tauri::command]
pub async fn get_user_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<User>> {
    let pool = &state.pool;
    let user = User::get_by_id(pool, id).await?;
    if user.is_none() {
        return Ok(None);
    }
    let mut user = user.unwrap();

    let store_id = utils::get_user_id(&state).await?;
    // cal balance
    let coupons = UserCoupon::find_by_user_id(pool, store_id, id).await?;
    user.balance = coupons
        .iter()
        .filter(|c| {
            if let Some(coupon) = c.coupon.as_ref() {
                if let Some(valid_to) = coupon.valid_to {
                    return coupon.coupon_type == Some(CouponType::StoredValueCard)
                        && utils::get_now() <= valid_to
                        && c.available_value.is_some()
                        && c.available_value.unwrap() > 0.;
                }
            }
            false
        })
        .map(|c| c.available_value.unwrap_or_default())
        .sum();

    Ok(Some(user))
}

#[tauri::command]
pub async fn get_user_by_ids(state: State<'_, AppState>, ids: Vec<i64>) -> Result<Vec<User>> {
    User::list_by_ids(&state.pool, &ids).await
}

#[tauri::command]
pub async fn get_user_by_cloth_code(
    state: State<'_, AppState>,
    code: String,
) -> Result<Option<User>> {
    let store_id = utils::get_user_id(&state).await?;
    User::get_user_by_cloth_code(&state.pool, store_id, &code).await
}

#[tauri::command]
pub async fn create_user(state: State<'_, AppState>, mut user: User) -> Result<User> {
    let store_id = utils::get_user_id(&state).await?;
    user.store_id = Some(store_id);
    user.create_user(&state).await
}

#[tauri::command]
pub async fn update_user(state: State<'_, AppState>, mut user: User) -> Result<bool> {
    let store_id = utils::get_user_id(&state).await?;
    user.store_id = Some(store_id);
    user.update_user(&state).await
}

#[tauri::command]
pub async fn change_user_status(
    state: State<'_, AppState>,
    user_id: i64,
    status: &str,
) -> Result<bool> {
    let mut user = User::get_by_id(&state.pool, user_id)
        .await?
        .ok_or(Error::not_found("用户未找到"))?;

    user.status = Some(status.to_string());
    let mut tr = state.pool.begin().await?;

    let result = user.update(&mut tr).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn change_user_identify(
    state: State<'_, AppState>,
    user_id: i64,
    identify: &str,
) -> Result<bool> {
    let mut user = User::get_by_id(&state.pool, user_id)
        .await?
        .ok_or(Error::not_found("用户未找到"))?;

    user.identify = Some(identify.to_string());
    let mut tr = state.pool.begin().await?;

    let result = user.update(&mut tr).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_users(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    User::delete_users(&state.pool, ids).await
}
