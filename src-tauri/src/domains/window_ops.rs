use tauri::{LogicalSize, Size, Window};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

#[tauri::command]
pub fn app_toggle_maximize(window: Window) -> Result<bool, String> {
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
pub fn app_minimize(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn app_close(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn app_is_maximized(window: Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn app_set_window_size(window: Window, width: u32, height: u32) -> Result<(), String> {
    window
        .set_size(Size::Logical(LogicalSize {
            width: width as f64,
            height: height as f64,
        }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn app_get_window_size(window: Window) -> Result<WindowDimensions, String> {
    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale_factor = window.scale_factor().unwrap_or(1.0);
    let logical = size.to_logical::<f64>(scale_factor);
    Ok(WindowDimensions {
        width: logical.width.round() as u32,
        height: logical.height.round() as u32,
    })
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn focus_emulator_window(hwnd_str: String) -> Result<(), String> {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        IsWindow, SetForegroundWindow, ShowWindow, SW_RESTORE,
    };

    let hwnd_val = hwnd_str.parse::<isize>().map_err(|e| e.to_string())?;
    let hwnd = hwnd_val as HWND;
    unsafe {
        if IsWindow(hwnd) == 0 {
            return Err("Window handle is invalid or closed".into());
        }
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn focus_emulator_window(_hwnd_str: String) -> Result<(), String> {
    Ok(())
}
