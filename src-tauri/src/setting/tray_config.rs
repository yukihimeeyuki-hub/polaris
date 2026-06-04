use tauri::{AppHandle, Manager,Wry};

pub struct TrayMenuConfig{
    pub id:&'static str,
    pub title:&'static str,
    pub enabled: bool,
    pub action: Box<dyn Fn(&AppHandle<Wry>) + Send + Sync>,
}
impl TrayMenuConfig {
    /// 提供一个构造函数，方便创建配置
    pub fn new(
        id: &'static str,
        title: &'static str,
        enabled: bool,
        action: impl Fn(&AppHandle<Wry>) + Send + Sync + 'static
    ) -> Self {
        Self {
            id,
            title,
            enabled,
            action: Box::new(action),
        }
    }
}
pub fn get_tray_menu_configs() -> Vec<TrayMenuConfig> {
    vec![
        TrayMenuConfig::new("show", "显示主窗口", true, |app| {
            if let Some(window) = app.get_webview_window("MainWindow") {
                window.show().unwrap();
                window.unminimize().unwrap();
                window.set_focus().unwrap();
            }
        }),
        TrayMenuConfig::new("settings", "设置", true, |_app| {
            println!("打开设置面板...");
            // 或者通过 app.emit("open-settings", ()) 通知前端
        }),

        TrayMenuConfig::new("quit", "退出应用", true, |_app| {
            std::process::exit(0);
        }),
    ]
}