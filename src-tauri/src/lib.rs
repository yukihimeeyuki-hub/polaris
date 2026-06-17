mod http;
mod setting;
mod window;
use http::commands::init_http_client;
use tauri::Manager;
use window::tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(init_http_client())
        .invoke_handler(tauri::generate_handler![
            http::commands::http_request,
            http::commands::http_get,
            http::commands::http_post,
            http::commands::http_put,
            http::commands::http_delete,
            http::commands::http_head,
            http::commands::http_options,
        ])
        .setup(|app| {
            let window = app.get_webview_window("StatusWindow").unwrap();

            // 设置窗口置顶
            window.set_always_on_top(true).unwrap();

            let window_clone = window.clone();
            if let Ok(Some(monitor)) = window.current_monitor() {
                let screen_size = monitor.size();
                if let Ok(window_size) = window.outer_size() {
                    let new_x = screen_size.width as i32 - window_size.width as i32;
                    let new_y = screen_size.height as i32 - window_size.height as i32;
                    let _ =
                        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                            x: new_x - 100,
                            y: new_y - 180,
                        }));
                }
            }
            window.show().unwrap();
            // 监听窗口移动事件
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Moved(_) = event {
                    // 获取当前显示器
                    if let Ok(Some(monitor)) = window_clone.current_monitor() {
                        let screen_size = monitor.size();
                        if let Ok(window_size) = window_clone.outer_size() {
                            // 计算右下角坐标
                            let new_x =
                                (screen_size.width as i32).saturating_sub(window_size.width as i32);
                            let new_y = (screen_size.height as i32)
                                .saturating_sub(window_size.height as i32);

                            // 设置回右下角
                            let _ = window_clone.set_position(tauri::Position::Physical(
                                tauri::PhysicalPosition {
                                x: new_x - 100,
                                                          y: new_y - 180,
                                },
                            ));
                        }
                    }
                }
            });
            // 初始化托盘
            create_tray(app.handle())?;
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
