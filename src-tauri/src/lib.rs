// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::Database;
use tauri::generate_handler;
use tauri::generate_context;
use tauri::AppHandle;
use tauri::Manager;
use std::env;
use api::ApiClient;

pub mod api;
pub mod utils;
pub mod auth;
pub mod cursor_reset;
pub mod tray;
pub mod database;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = show_window(app);
        }))
        .setup(|app| {
            let db = Database::new(&app.handle()).expect("数据库初始化失败");
            app.manage(db);
            
            let api_client = ApiClient::new(Some(app.handle().clone()));
            app.manage(api_client);
            
            tray::setup_system_tray(app)?;
            Ok(())
        })
        .invoke_handler(generate_handler![
            api::login,
            api::get_user_info,
            api::activate,
            api::change_password,
            api::logout,
            api::get_account,
            api::get_usage,
            api::check_user,
            api::send_code,
            api::get_version,
            api::get_public_info,
            api::reset_password,
            api::register,
            api::set_user_data,
            api::get_user_data,
            api::del_user_data,

            cursor_reset::commands::reset_machine_id,
            cursor_reset::commands::switch_account,
            cursor_reset::commands::get_machine_ids,
            cursor_reset::commands::check_cursor_running,
            cursor_reset::commands::check_admin_privileges,
            cursor_reset::commands::is_hook,
            cursor_reset::commands::hook_main_js,
            cursor_reset::commands::restore_hook,
            cursor_reset::commands::check_is_windows,
            cursor_reset::commands::close_cursor,
            cursor_reset::commands::launch_cursor,
            cursor_reset::commands::find_cursor_path,
        ])
        .run(generate_context!())
        .expect("error while running tauri application")
}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}