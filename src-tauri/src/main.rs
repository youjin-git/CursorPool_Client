// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cursor_pool_lib::*;
use dotenv::dotenv;
use tauri::Builder;
use tauri::generate_handler;
use tauri::generate_context;

fn main() {
    dotenv().ok();  // 加载 .env 文件
    Builder::default()
        .setup(|app| {
            tray::setup_system_tray(app)?;
            Ok(())
        })
        .invoke_handler(generate_handler![
            api::login,
            api::get_user_info,
            api::activate,
            api::change_password,
            api::get_account,
            api::get_usage,
            api::get_user_info_cursor,
            api::check_user,
            api::send_code,
            api::get_version,
            api::get_public_info,
            reset_machine_id,
            switch_account,
            get_current_account,
            get_machine_ids,
            cursor_reset::commands::check_cursor_running,
            cursor_reset::commands::kill_cursor_process,
        ])
        .manage(api::ApiClient::default())
        .run(generate_context!())
        .expect("error while running tauri application");
}