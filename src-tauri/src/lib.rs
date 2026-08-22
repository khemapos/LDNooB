use tauri::{LogicalSize, Size, Window};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn app_toggle_maximize(window: Window) -> Result<bool, String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
fn app_minimize(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
fn app_close(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
fn app_is_maximized(window: Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

#[tauri::command]
fn app_set_window_size(window: Window, width: u32, height: u32) -> Result<(), String> {
    window
        .set_size(Size::Logical(LogicalSize {
            width: width as f64,
            height: height as f64,
        }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn app_get_window_size(window: Window) -> Result<WindowDimensions, String> {
    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale_factor = window.scale_factor().unwrap_or(1.0);
    let logical = size.to_logical::<f64>(scale_factor);
    Ok(WindowDimensions {
        width: logical.width.round() as u32,
        height: logical.height.round() as u32,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            app_toggle_maximize,
            app_minimize,
            app_close,
            app_is_maximized,
            app_set_window_size,
            app_get_window_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
