pub mod config;
pub mod db;

pub mod error;
pub mod printer;
pub mod tray;
pub mod utils;
pub mod sql;

use tauri::generate_handler;

use db::user;
use db::tags;
use db::clothing;
use db::drying_rack;
use db::cloth_price;
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
            // tags
            tags::list_pagination,
            tags::list_all,
            tags::add_tag,
            tags::get_tag_by_id,
            tags::update_tag,
            tags::soft_delete_tag,
            tags::update_ref_num,
            tags::tag_name_exists,
            tags::delete_tags_batch,
            tags::change_tag_status,
            // clothing
            clothing::list_clothing_pagination,
            clothing::list_clothing_all,
            clothing::add_clothing,
            clothing::get_clothing_by_id,
            clothing::update_clothing,
            clothing::soft_delete_clothing,
            clothing::delete_clothing_batch,
            clothing::update_clothing_ref_num,
            clothing::clothing_name_exists,
            // rack
            drying_rack::list_rack_all,
            drying_rack::add_rack,
            drying_rack::get_rack_by_id,
            drying_rack::update_rack,
            drying_rack::delete_racks,
            // cloth price
            cloth_price::add_cloth_price,
            cloth_price::get_cloth_price,
            cloth_price::update_cloth_price,
            cloth_price::delete_cloth_prices,
            cloth_price::list_cloth_prices_pagination,
            cloth_price::list_cloth_prices_by_order_type,
            cloth_price::update_cloth_price_status,
            cloth_price::update_cloth_price_ref_num
        ])
        .build(tauri::generate_context!())
        .expect("error while building Tauri application")
}
