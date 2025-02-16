use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::db::{AppState, Curd, Validator};
use crate::error::{Error, Result};
use crate::routers::{MetaVo, RouterVo};
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Menu {
    pub menu_id: Option<i64>,                       // 菜单ID
    pub menu_name: Option<String>,                  // 菜单名称
    pub parent_id: Option<i64>,                     // 父菜单ID
    pub order_num: Option<i32>,                     // 显示顺序
    pub path: Option<String>,                       // 路由地址
    pub component: Option<String>,                  // 组件路径
    pub query: Option<String>,                      // 路由参数
    pub route_name: Option<String>,                 // 路由名称
    pub is_frame: Option<String>,                   // 是否为外链（0是 1否）
    pub is_cache: Option<String>,                   // 是否缓存（0缓存 1不缓存）
    pub menu_type: Option<String>,                  // 菜单类型（M目录 C菜单 F按钮）
    pub visible: Option<String>,                    // 菜单状态（0显示 1隐藏）
    pub status: Option<String>,                     // 菜单状态（0正常 1停用）
    pub perms: Option<String>,                      // 权限标识
    pub icon: Option<String>,                       // 菜单图标
    pub create_by: Option<String>,                  // 创建者
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间
    pub update_by: Option<String>,                  // 更新者
    pub update_time: Option<DateTime<FixedOffset>>, // 更新时间
    pub remark: Option<String>,                     // 备注
    pub children: Vec<Menu>,
}

impl FromRow<'_, SqliteRow> for Menu {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            menu_id: row.try_get("menu_id").unwrap_or_default(),
            menu_name: row.try_get("menu_name").unwrap_or_default(),
            parent_id: row.try_get("parent_id").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            path: row.try_get("path").unwrap_or_default(),
            component: row.try_get("component").unwrap_or_default(),
            query: row.try_get("query").unwrap_or_default(),
            route_name: row.try_get("route_name").unwrap_or_default(),
            is_frame: row.try_get("is_frame").unwrap_or_default(),
            is_cache: row.try_get("is_cache").unwrap_or_default(),
            menu_type: row.try_get("menu_type").unwrap_or_default(),
            visible: row.try_get("visible").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            perms: row.try_get("perms").unwrap_or_default(),
            icon: row.try_get("icon").unwrap_or_default(),
            create_by: row.try_get("create_by").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_by: row.try_get("update_by").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            children: Vec::new(),
        })
    }
}

impl Validator for Menu {
    fn validate(&self) -> Result<()> {
        if self.menu_name.is_none() {
            return Err(Error::bad_request("菜单名称不能为空"));
        }

        Ok(())
    }
}

impl Curd for Menu {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM menu ";
    const QUERY_SQL: &'static str = "SELECT * FROM menu WHERE 1=1 ";
    const BY_ID_SQL: &'static str = "SELECT * FROM menu WHERE menu_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM menu WHERE menu_id IN ( ";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(menu_name) = &self.menu_name {
            builder
                .push(" AND menu_name LIKE ")
                .push_bind(format!("%{}%", menu_name));
        }

        if let Some(visible) = &self.visible {
            builder.push(" AND visible = ").push_bind(visible);
        }

        if let Some(status) = &self.status {
            builder.push(" AND status = ").push_bind(status);
        }
    }
}

impl Menu {
    pub async fn select_menu_tree_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let query = r#"
        SELECT DISTINCT
            menu_id,
            parent_id,
            menu_name,
            path,
            component,
            query,
            route_name,
            visible,
            status,
            IFNULL(perms, '') AS perms,
            is_frame,
            is_cache,
            menu_type,
            icon,
            order_num,
            create_time
        FROM
            menu m
        WHERE
            menu_type IN ('M', 'C')
            AND status = '0'
        ORDER BY
            parent_id,
            order_num
    "#;

        let menus = sqlx::query_as(query).fetch_all(pool).await?;

        tracing::debug!("select_menu_tree_all: {:?}", menus);
        Ok(menus)
    }

    // select count(1) from menu where parent_id = #{menuId}
    pub async fn has_child_by_id(pool: &Pool<Sqlite>, menu_id: i64) -> Result<bool> {
        let query = r#"SELECT COUNT(1) FROM menu WHERE parent_id = ?"#;

        let count = sqlx::query_scalar::<_, u64>(query)
            .bind(menu_id)
            .fetch_one(pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn check_menu_name_unique(
        pool: &Pool<Sqlite>,
        menu_name: &str,
        parent_id: i64,
    ) -> Result<Option<Self>> {
        let query = r#"
        SELECT
            menu_id,
            parent_id,
            menu_name,
            path,
            component,
            query,
            route_name,
            visible,
            status,
            IFNULL(perms, '') AS perms,
            is_frame,
            is_cache,
            menu_type,
            icon,
            order_num,
            create_time
        FROM
            menu m
        WHERE
            menu_name = ?
            AND parent_id = ?
        LIMIT 1
    "#;

        let menu = sqlx::query_as(query)
            .bind(menu_name)
            .bind(parent_id)
            .fetch_optional(pool)
            .await?;

        Ok(menu)
    }

    // insert
    pub async fn create_menu(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let query = r#"
        INSERT INTO menu (menu_name, parent_id, order_num, path, component,
        query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_time)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)RETURNING *"#;
        let result = sqlx::query_as(query)
            .bind(&self.menu_name)
            .bind(&self.parent_id)
            .bind(&self.order_num)
            .bind(&self.path)
            .bind(&self.component)
            .bind(&self.query)
            .bind(&self.route_name)
            .bind(&self.is_frame)
            .bind(&self.is_cache)
            .bind(&self.menu_type)
            .bind(&self.visible)
            .bind(&self.status)
            .bind(&self.perms)
            .bind(&self.icon)
            .bind(utils::get_now())
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // update
    pub async fn update_menu(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let query = r#"
        UPDATE menu SET menu_name = ?, parent_id = ?, order_num = ?,
        path = ?, component = ?, query = ?, route_name = ?, is_frame = ?,
        is_cache = ?, menu_type = ?, visible = ?, status = ?, perms = ?,
        icon = ?, update_time = ?
        WHERE menu_id = ?
   "#;
        let result = sqlx::query(query)
            .bind(&self.menu_name)
            .bind(&self.parent_id)
            .bind(&self.order_num)
            .bind(&self.path)
            .bind(&self.component)
            .bind(&self.query)
            .bind(&self.route_name)
            .bind(&self.is_frame)
            .bind(&self.is_cache)
            .bind(&self.menu_type)
            .bind(&self.visible)
            .bind(&self.status)
            .bind(&self.perms)
            .bind(&self.icon)
            .bind(utils::get_now())
            .bind(&self.menu_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_menu(pool: &Pool<Sqlite>, menu_id: i64) -> Result<bool> {
        let query = r#"DELETE FROM menu WHERE menu_id = ?"#;
        let result = sqlx::query(query).bind(menu_id).execute(pool).await?;
        Ok(result.rows_affected() > 0)
    }
}

// service
impl Menu {
    pub async fn get_menu_tree(pool: &Pool<Sqlite>) -> Result<Vec<Menu>> {
        let mut menus = Menu::select_menu_tree_all(pool).await?;
        Ok(Self::get_child_perms(&mut menus, 0))
    }

    pub fn get_child_perms(list: &[Menu], parent_id: i64) -> Vec<Menu> {
        let mut return_list = Vec::new();

        for item in list.iter() {
            if item.parent_id == Some(parent_id) {
                let mut node = item.clone();
                Menu::recursion_fn(list, &mut node); // 使用不可变借用
                return_list.push(node);
            }
        }

        return_list
    }

    /// 递归列表
    pub fn recursion_fn(list: &[Menu], t: &mut Menu) {
        let child_list = Menu::get_child_list(list, t);
        t.children = child_list.clone();
        for child in t.children.iter_mut() {
            if Menu::has_child(list, child) {
                Menu::recursion_fn(list, child);
            }
        }
    }

    pub fn get_child_list(list: &[Menu], t: &Menu) -> Vec<Menu> {
        list.iter()
            .filter(|&item| item.parent_id == t.menu_id)
            .cloned()
            .collect()
    }

    pub fn has_child(list: &[Menu], t: &Menu) -> bool {
        Menu::get_child_list(list, t).is_empty()
    }

    pub fn build_menus(menus: Vec<Menu>) -> Vec<RouterVo> {
        let mut routers = Vec::new();

        for menu in menus {
            tracing::debug!("menu_route name: {:?}", menu.get_route_name());
            let mut router = RouterVo::new(
                menu.get_route_name(),
                menu.get_router_path(),
                menu.get_component(),
                menu.query.clone().unwrap_or_default(),
                menu.visible.as_deref() == Some("1"), // Hidden if visible is "1"
            );

            router.meta = Some(MetaVo::new(
                menu.menu_name.clone().unwrap_or_default(),
                menu.icon.clone().unwrap_or_default(),
                None,
            ));

            tracing::debug!("router: {:?}", router);
            let children = menu.children.clone();
            if !children.is_empty() && Some("M") == menu.menu_type.as_deref() {
                // If it has children, mark as alwaysShow and build children recursively
                router.always_show = Some(true);
                router.redirect = "noRedirect".to_string();
                router.children = Self::build_menus(children); // Recursive call for children
            } else if menu.is_menu_frame() {
                router.meta = None;
                router.children.clear();
                // let mut children_list = Vec::new();
                // If it's a frame-based menu, add a direct child
                let mut child_router = RouterVo::new(
                    menu.get_route_name(),
                    menu.path.clone().unwrap_or_default(),
                    menu.component.clone().unwrap_or_default(),
                    menu.query.clone().unwrap_or_default(),
                    menu.visible.as_deref() == Some("1"),
                );
                child_router.meta = Some(MetaVo::new(
                    menu.menu_name.unwrap_or_default(),
                    menu.icon.unwrap_or_default(),
                    None,
                ));
                router.children.push(child_router);
            } else if menu.parent_id == Some(0) && menu.is_inner_link() {
                router.children.clear();
                router.meta = Some(MetaVo::new(
                    menu.menu_name.clone().unwrap(),
                    menu.icon.clone().unwrap(),
                    None,
                ));
                router.path = "/".to_string();
                // If it's an inner link, update path and component
                let inner_path =
                    Self::inner_link_replace_each(&menu.path.clone().unwrap_or_default());
                let mut child_router = RouterVo::new(
                    menu.get_route_name(),
                    inner_path,
                    "InnerLink".to_string(),
                    String::new(),
                    menu.visible.as_deref() != Some("1"),
                );
                child_router.component = "InnerLink".to_string();
                child_router.meta = Some(MetaVo::new(
                    menu.menu_name.unwrap_or_default(),
                    menu.icon.unwrap_or_default(),
                    None,
                ));
                router.children.push(child_router);
            }

            routers.push(router);
        }

        routers
    }
    fn get_component(&self) -> String {
        let mut component = "Layout".to_string();

        if let Some(comp) = &self.component {
            if !self.is_menu_frame() {
                component = comp.clone();
            }
        } else if self.parent_id != Some(0) && self.is_inner_link() {
            component = "InnerLink".to_string();
        } else if self.component.is_none() && self.is_parent_view() {
            component = "ParentView".to_string();
        }

        component
    }

    fn is_parent_view(&self) -> bool {
        self.parent_id != Some(0) && self.menu_type.as_deref() == Some("M")
    }

    fn get_route_name(&self) -> String {
        // 非外链并且是一级目录（类型为目录）
        if self.is_menu_frame() {
            return String::new();
        }
        Self::get_route_name_from(self.route_name.as_deref(), self.path.as_deref())
    }

    /// 获取路由名称，如没有配置路由名称则取路由地址
    fn get_route_name_from(name: Option<&str>, path: Option<&str>) -> String {
        let router_name = if name.is_none() || name.unwrap().is_empty() {
            path.unwrap_or_default()
        } else {
            name.unwrap()
        };
        Self::capitalize(router_name)
    }

    /// 将字符串首字母大写（模拟 StringUtils.capitalize）
    fn capitalize(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut chars = s.chars();
        chars
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_default()
            + chars.as_str()
    }

    fn get_router_path(&self) -> String {
        tracing::debug!("menu: {:?}", self);
        let mut router_path = self.path.clone().unwrap_or_default();

        // 内链打开外网方式
        if self.parent_id != Some(0) && self.is_inner_link() {
            router_path = Self::inner_link_replace_each(&router_path);
        }

        // 非外链并且是一级目录（类型为目录）
        if self.parent_id == Some(0)
            && self.menu_type == Some("M".to_string())
            && self.is_frame == Some("1".to_string())
        {
            router_path = format!("/{}", self.path.clone().unwrap_or_default());
        }
        // 非外链并且是一级目录（类型为菜单）
        else if self.is_menu_frame() {
            router_path = "/".to_string();
        }

        router_path
    }

    // Check if the menu is a frame-based menu
    fn is_menu_frame(&self) -> bool {
        self.parent_id == Some(0)
            && self.menu_type == Some("C".to_string())
            && self.is_frame == Some("1".to_string())
    }

    // Check if the menu is an inner link
    fn is_inner_link(&self) -> bool {
        self.is_frame == Some("1".to_string())
            && self
                .path
                .as_ref()
                .map_or(false, |path| path.starts_with("http"))
    }

    // Replace special characters in inner links
    fn inner_link_replace_each(path: &str) -> String {
        let search = ["http://", "https://", "www.", ".", ":"];
        let replace = ["", "", "", "/", "/"];

        let mut result = path.to_string();

        for (s, r) in search.iter().zip(replace.iter()) {
            result = result.replace(s, r);
        }

        result
    }
}

#[tauri::command]
pub async fn get_menu_list(state: State<'_, AppState>, menu: Menu) -> Result<Vec<Menu>> {
    menu.get_all(&state.pool).await
}

#[tauri::command]
pub async fn get_menu_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Menu>> {
    Menu::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn add_menu(state: State<'_, AppState>, menu: Menu) -> Result<Menu> {
    menu.create_menu(&state.pool).await
}

#[tauri::command]
pub async fn update_menu(state: State<'_, AppState>, menu: Menu) -> Result<bool> {
    menu.validate()?;
    // check name unique
    if let Some(result) = Menu::check_menu_name_unique(
        &state.pool,
        menu.menu_name.as_ref().unwrap(),
        *menu.parent_id.as_ref().unwrap(),
    )
    .await?
    {
        if result.menu_id != menu.menu_id {
            return Err(Error::bad_request(format!(
                "菜单名称已存在：{}",
                &result.menu_name.unwrap()
            )));
        }
    }

    if menu.menu_id == menu.parent_id {
        return Err(Error::bad_request("上级菜单不能为自身"));
    }
    menu.update_menu(&state.pool).await
}

#[tauri::command]
pub async fn delete_menu(state: State<'_, AppState>, id: i64) -> Result<bool> {
    let pool = &state.pool;
    // if it has child menu
    if Menu::has_child_by_id(pool, id).await? {
        return Err(Error::internal("菜单存在子菜单，无法删除"));
    }
    Menu::delete_menu(pool, id).await
}
