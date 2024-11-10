pub mod config;
pub mod db;

pub mod error;
pub mod printer;
pub mod tray;
pub mod utils;

use tauri::generate_handler;

use db::user;
use db::tags;
use tray::create_tray;

pub fn create_app<R: tauri::Runtime, T: Send + Sync + 'static>(
    builder: tauri::Builder<R>,
    state: T,
) -> tauri::App<R> {
    builder
        // .plugin(tauri_plugin_log::Builder::default().build())
        .manage(state)
        .setup(|app| {
            create_tray(app.handle(), true)?;
            Ok(())
        })
        .invoke_handler(generate_handler![
            // update_tray_menu,
            user::insert_user,
            user::get_users,
            printer::print,
            db::printer::get_printers,
            db::printer::set_printer,
            db::printer::get_settled_printer,
            tags::add_tag,
            tags::get_tag_by_id,
            tags::update_tag,
            tags::soft_delete_tag,
            tags::increase_ref_num,
            tags::tag_name_exists,
            tags::delete_tags_batch
        ])
        .build(tauri::generate_context!())
        .expect("error while building Tauri application")
}
