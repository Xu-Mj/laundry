pub mod captcha;
pub mod config;
pub mod db;
pub mod error;
pub mod files;
pub mod home;
pub mod pay;
pub mod printer;
pub mod state;
pub mod tray;
pub mod update;
pub mod utils;

use tauri::Runtime;
use tauri::ipc::Invoke;
use tauri_plugin_fs::FsExt;

use crate::db::{
    alipay_config, cloth_price, clothing, clothing_category, clothing_style, configs, coupons,
    delivery, dict_data, dict_type, drying_rack, expenditure, local_users, membership_level,
    message, notice_temp, order_clothes, orders, payments, qrcode_payments, subscription_service,
    subscriptions, tags, user, user_coupons, user_tours, wechat_config,
};

pub fn create_app<R: tauri::Runtime, T: Send + Sync + 'static>(
    builder: tauri::Builder<R>,
    state: T,
) -> tauri::App<R> {
    builder
        .manage(state)
        .setup(|app| {
            #[cfg(debug_assertions)] // 仅在调试构建时包含此代码
            {
                use tauri::Manager;
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            // allowed the given directory
            let scope = app.fs_scope();
            scope
                .allow_directory("/path/to/directory", false)
                .expect("msg");

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update::update(handle).await.unwrap();
            });
            Ok(())
        })
        .invoke_handler(handle_command)
        // Add device management handlers
        // .invoke_handler(tauri::generate_handler![
        //     db::store_devices::get_device_info,
        //     db::store_devices::validate_login_device,
        // ])
        .build(tauri::generate_context!())
        .expect("error while building Tauri application")
}

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
        files::save_file,
        files::delete_file,
        // captcha
        captcha::get_captcha,
        // alipay configuration
        alipay_config::save_alipay_config,
        alipay_config::get_alipay_config,
        alipay_config::deactivate_alipay,
        // qrcode payment
        qrcode_payments::get_qrcode_payment_by_pay_id,
        qrcode_payments::get_qrcode_payment_by_trade_no,
        qrcode_payments::get_qrcode_payment_by_out_trade_no,
        // wechat pay configuration
        wechat_config::save_wechat_config,
        wechat_config::get_wechat_config,
        wechat_config::deactivate_wechat,
        // home
        home::query_total_count,
        home::query_count,
        home::query_chart,
        home::fetch_monthly_payment_summary,
        // clothing category
        clothing_category::list_clothing_category_pagination,
        clothing_category::list_clothing_category_all,
        clothing_category::get_clothing_category_by_id,
        clothing_category::add_clothing_category,
        clothing_category::update_clothing_category,
        clothing_category::delete_clothing_category_batch,
        // clothing style
        clothing_style::list_clothing_style_pagination,
        clothing_style::list_clothing_style_all,
        clothing_style::get_clothing_style_by_id,
        clothing_style::get_clothing_style_by_category_id,
        clothing_style::add_clothing_style,
        clothing_style::update_clothing_style,
        clothing_style::delete_clothing_style_batch,
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
        clothing::create_clothing_4_create_order,
        clothing::get_clothing_by_id,
        clothing::update_clothing,
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
        // New delivery-related commands for order_clothes
        order_clothes::list_delivery_eligible_clothes,
        order_clothes::deliver_clothes,
        order_clothes::get_order_cloths_by_ids,
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
        dict_type::get_dict_type_by_type,
        dict_type::add_dict_type,
        dict_type::update_dict_type,
        dict_type::delete_dict_types,
        // dict_data
        dict_data::get_dict_data_list,
        dict_data::get_by_dict_type,
        dict_data::get_dict_data_by_code,
        dict_data::add_dict_data,
        dict_data::add_dict_data_auto,
        dict_data::update_dict_data,
        dict_data::delete_dict_data,
        // local_users
        local_users::get_info,
        local_users::validate_pwd,
        local_users::guest_login,
        local_users::login,
        local_users::logout,
        local_users::update_local_user,
        local_users::update_pwd,
        local_users::get_device_info,
        local_users::get_sms_verification_code,
        local_users::register,
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
        // subscription
        subscriptions::create_subscription,
        subscriptions::update_subscription,
        subscriptions::get_user_subscriptions,
        subscriptions::get_active_by_user_id,
        // tours
        user_tours::update_tour_guide,
        user_tours::check_tour_completed,
        // New delivery commands
        delivery::create_delivery,
        delivery::complete_delivery,
        delivery::cancel_delivery,
        delivery::get_delivery_by_id,
        delivery::list_deliveries,
        // messages
        message::save_message,
        message::get_messages,
        message::delete_message,
        message::get_unread_message_count,
        message::mark_messages_as_read,
        message::clear_messages,
        // Subscription service commands
        subscription_service::get_all_plans,
        subscription_service::get_recommended_plans,
        subscription_service::get_plans_by_type,
        subscription_service::get_plan_by_id,
        subscription_service::get_subscription_by_id,
        subscription_service::cancel_subscription,
        subscription_service::check_store_subscription,
        subscription_service::get_sms_plans,
        subscription_service::get_alipay_qr_code,
        subscription_service::check_alipay_qr_code_payment_status,
    ];
    handler(invoke);

    true
}
