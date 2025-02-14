// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    WebviewWindow,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn show_and_focus_window(window: &WebviewWindow) {
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let _ = window.set_always_on_top(false);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "退出程序").build(app)?;
            let show = MenuItemBuilder::with_id("show", "显示界面").build(app)?;
            let switch_account = MenuItemBuilder::with_id("switch_account", "一键换号").build(app)?;
            let switch_account_manual = MenuItemBuilder::with_id("switch_account_manual", "切换账号").build(app)?;
            let switch_machine = MenuItemBuilder::with_id("switch_machine", "换机器码").build(app)?;
            
            let menu = MenuBuilder::new(app)
                .items(&[
                    &switch_account,
                    &switch_account_manual,
                    &switch_machine,
                    &show,
                    &quit,
                ])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)  // disable showing menu on left click
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            show_and_focus_window(&window);
                        }
                    }
                    "switch_account" => {
                        println!("一键换号");
                    }
                    "switch_account_manual" => {
                        println!("手动换号");
                    }
                    "switch_machine" => {
                        println!("换机器码");
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button, button_state, .. } = event {
                        match (button, button_state) {
                            (MouseButton::Left, MouseButtonState::Up) => {
                                let app = tray.app_handle();
                                if let Some(window) = app.get_webview_window("main") {
                                    show_and_focus_window(&window);
                                }
                            }
                            // show menu
                            _ => {}
                        }
                    }
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
