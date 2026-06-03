// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod http;
mod fs;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .setup(|app| {
            // 1. 创建菜单项
            let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
            // 3. 构建托盘图标
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone()) // 设置托盘图标
                .tooltip("北极星") // 鼠标悬停提示
                .menu(&menu) // 绑定右键菜单
                .show_menu_on_left_click(false) // 【核心】关闭左键单击弹出菜单的默认行为
                .on_menu_event(|app, event| {
                    // 处理菜单项的点击事件
                    match event.id().as_ref() {
                        "show" => {
                            let window = app.get_webview_window("main").unwrap();
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // 4. 【核心】监听托盘图标的鼠标点击事件
                    if let TrayIconEvent::Click {
                        button,
                        button_state: MouseButtonState::Up, // 监听鼠标抬起动作
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let window = app.get_webview_window("main").unwrap();

                        match button {
                            // 左键单击：显示主窗口并置顶
                            MouseButton::Left => {
                                if !window.is_visible().unwrap() {
                                    window.show().unwrap();
                                }
                                // 如果窗口被最小化了，先恢复
                                if window.is_minimized().unwrap() {
                                    window.unminimize().unwrap();
                                }
                                window.set_focus().unwrap();
                            }
                            // 右键单击：Tauri 默认就会弹出 .menu() 绑定的菜单
                            // 如果你需要自定义右键逻辑（比如弹出无边框的自定义菜单窗口），可以在这里处理
                            MouseButton::Right => {
                                println!("右键点击了托盘图标");
                            }
                            _ => {}
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
         .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
