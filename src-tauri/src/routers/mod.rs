use crate::db::menu::Menu;
use crate::db::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterVo {
    pub name: String,
    pub path: String,
    pub hidden: bool,
    pub redirect: String,
    pub component: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show: Option<bool>,
    pub meta: Option<MetaVo>,
    pub children: Vec<RouterVo>,
}

impl RouterVo {
    pub fn new(name: String, path: String, component: String, query: String, hidden: bool) -> Self {
        RouterVo {
            name,
            path,
            hidden,
            redirect: String::new(),
            component,
            query,
            always_show: None,
            meta: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaVo {
    pub title: String,
    pub icon: String,
    pub no_cache: bool,
    pub link: Option<String>,
}

impl MetaVo {
    pub fn new(title: String, icon: String, link: Option<String>) -> Self {
        MetaVo {
            title,
            icon,
            no_cache: false,
            link,
        }
    }
}

#[tauri::command]
pub async fn get_routers(state: tauri::State<'_, AppState>) -> crate::error::Result<Vec<RouterVo>> {
    let menus = Menu::get_menu_tree(&state.pool).await?;
    Ok(Menu::build_menus(menus))
}
