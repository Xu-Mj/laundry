pub mod config;
pub mod db;

pub mod captcha;
pub mod error;
pub mod files;
pub mod home;
pub mod printer;
pub mod routers;
pub mod sql;
pub mod tray;
pub mod utils;

use tauri::ipc::Invoke;
use tauri::Runtime;
use tauri_plugin_fs::FsExt;

use crate::db::{
    cloth_price, clothing, configs, coupons, dict_data, dict_type, drying_rack, expenditure,
    local_users, membership_level, menu, notice_temp, order_clothes, orders, payments, tags, user,
    user_coupons,
};

fn handle_command<R: Runtime>(invoke: Invoke<R>) -> bool {
    // 获取 StateManager
    // let state_manager = invoke.message.state();
    // let state: State<AppState> = state_manager.get();

    // 排除 login 命令，避免死循环
    // if invoke.message.command() != "login" {
    // if let Err(err) = state.check_auth() {
    // tracing::debug!("---------------------------------------------------------------------------------------111111111111{:?}", err);
    // invoke.resolver.reject(err);
    // return false;
    // }
    // }

    // 调用原始命令处理器
    let handler: fn(Invoke<R>) -> bool = tauri::generate_handler![
        // files
        files::save_image,
        files::delete_image,
        files::get_image,
        // captcha
        captcha::get_captcha,
        // login

        // home
        home::query_total_count,
        home::query_count,
        home::query_chart,
        home::fetch_monthly_payment_summary,
        home::fetch_payment_summary,
        // update_tray_menu,
        user::get_users_pagination,
        user::get_all_users,
        user::get_user_by_id,
        user::get_user_by_ids,
        user::get_user_by_cloth_code,
        user::create_user,
        user::update_user,
        user::change_user_status,
        user::delete_users,
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
        cloth_price::update_cloth_price_ref_num,
        // order clothes
        order_clothes::list_order_clothes,
        order_clothes::list_order_clothes_history,
        order_clothes::add_order_cloth,
        order_clothes::update_order_cloth,
        order_clothes::get_order_cloth_by_id,
        order_clothes::get_order_cloth_by_code,
        order_clothes::delete_order_cloth_by_ids,
        order_clothes::hang_order_cloth,
        order_clothes::pickup_order_cloth,
        order_clothes::remove_pic_from_order_cloth,
        order_clothes::upload_cloth_pic,
        // coupons
        coupons::add_coupon,
        coupons::update_coupon,
        coupons::get_coupon_list,
        coupons::get_coupons4sale,
        coupons::get_coupon_by_id,
        coupons::buy_coupons,
        coupons::gift_coupons,
        coupons::delete_coupons,
        // user coupons
        user_coupons::get_user_coupons,
        user_coupons::get_user_coupons4sale,
        user_coupons::get_user_coupon_by_user_id,
        // orders
        orders::create_order,
        orders::get_orders_pagination,
        orders::get_orders4home,
        orders::get_order_by_id,
        orders::update_order,
        orders::delete_orders,
        orders::update_adjust,
        orders::pay_order,
        orders::get_refund_info,
        orders::refund_order,
        orders::get_orders4history,
        orders::get_count_by_user_id,
        // payments
        payments::get_total_amount,
        // configs
        configs::add_config,
        configs::get_config_list,
        configs::get_config_by_id,
        configs::delete_configs,
        configs::update_config,
        configs::get_config_by_key,
        // dict_type
        dict_type::get_dict_type_list,
        dict_type::get_dict_type_all,
        dict_type::get_dict_type_by_id,
        dict_type::add_dict_type,
        dict_type::update_dict_type,
        dict_type::delete_dict_types,
        // dict_data
        dict_data::get_dict_data_list,
        dict_data::get_by_dict_type,
        dict_data::get_dict_data_by_code,
        dict_data::add_dict_data,
        dict_data::update_dict_data,
        dict_data::delete_dict_data,
        // menu
        menu::get_menu_list,
        menu::get_menu_by_id,
        menu::add_menu,
        menu::update_menu,
        menu::delete_menu,
        // routers
        routers::get_routers,
        // local_users
        local_users::get_info,
        local_users::login,
        local_users::logout,
        local_users::register,
        local_users::update_pwd,
        // expenditure
        expenditure::get_exp_pagination,
        expenditure::get_exp_by_id,
        expenditure::create_exp,
        expenditure::update_exp,
        expenditure::delete_exp,
        // notice
        notice_temp::get_temp_pagination,
        notice_temp::get_temp_by_id,
        notice_temp::create_temp,
        notice_temp::update_temp,
        notice_temp::delete_temp,
        notice_temp::get_notice_record_pagination,
        notice_temp::delete_all_record,
        notice_temp::delete_old_record,
        notice_temp::send_notice,
        // membership_level
        membership_level::get_membership_level_pagination,
        membership_level::get_membership_level_by_id,
        membership_level::get_membership_level_list,
        membership_level::create_membership_level,
        membership_level::update_membership_level,
        membership_level::delete_membership_level,
    ];
    handler(invoke);

    true
}

// async fn start_coupon_check_scheduler(pool: Arc<Pool<Sqlite>>) {
//     let mut interval = interval(Duration::from_secs(24 * 60 * 60)); // Check once per day

//     tokio::spawn(async move {
//         loop {
//             interval.tick().await;
//             match db::coupons::Coupon::check_expiring_coupons(&pool).await {
//                 Ok(_) => tracing::info!("Successfully checked expiring coupons"),
//                 Err(e) => tracing::error!("Failed to check expiring coupons: {}", e),
//             }
//         }
//     });
// }

pub fn create_app<R: tauri::Runtime, T: Send + Sync + 'static>(
    builder: tauri::Builder<R>,
    state: T,
) -> tauri::App<R> {
    builder
        .manage(state)
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope
                .allow_directory("/path/to/directory", false)
                .expect("msg");

            Ok(())
        })
        .invoke_handler(handle_command)
        .build(tauri::generate_context!())
        .expect("error while building Tauri application")
}
