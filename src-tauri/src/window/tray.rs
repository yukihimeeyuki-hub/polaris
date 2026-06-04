use crate::setting::tray_config;
use std::sync::Arc; // 引入 Arc

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Wry, // 引入 Wry，去掉 Runtime
};

// 1. 把这里的 <R: Runtime> 去掉，将 AppHandle<R> 改为 AppHandle<Wry>
pub fn create_tray(app: &AppHandle<Wry>) -> tauri::Result<()> {
    let configs = tray_config::get_tray_menu_configs();

    // 2. 批量生成真实的菜单项
    let mut menu_items = Vec::new();
    for config in &configs {
        let item = MenuItem::with_id(app, config.id, config.title, config.enabled, None::<&str>)?;
        menu_items.push(item);
    }

    // 3. 构建菜单 (动态转型的类型也改为 Wry)
    let menu = Menu::with_items(
        app,
        &menu_items.iter().map(|m| m as &dyn tauri::menu::IsMenuItem<Wry>).collect::<Vec<_>>()
    )?;

    // 4. 用 Arc 包裹配置，以便安全地在多线程/事件闭包中共享
    let configs_share = Arc::new(configs);

    // 5. 构建托盘图标
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("北极星")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            let clicked_id = event.id().as_ref();

            // 6. 从 Arc 中查找被点击的配置并执行
            if let Some(config) = configs_share.iter().find(|c| c.id == clicked_id) {
                (config.action)(&app); // 注意：这里传的是 &app 引用
            }
        })
        .build(app)?;

    Ok(())
}