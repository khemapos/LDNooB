use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Emulator {
    pub index: i32,
    pub name: String,
    pub top_hwnd: String,
    pub bind_hwnd: String,
    pub is_running: bool,
    pub pid: i32,
    pub vbox_pid: i32,
    pub width: i32,
    pub height: i32,
    pub dpi: i32,
    pub brand: String,
    pub model: String,
    pub imei: String,
    pub mac: String,
    pub android_id: String,
    pub disk_size_bytes: u64,
}

pub fn get_executable_path(ldplayer_dir: &str) -> Result<PathBuf, String> {
    let dir = Path::new(ldplayer_dir);
    if !dir.exists() {
        return Err(format!("Directory does not exist: {}", ldplayer_dir));
    }
    let direct_ld = dir.join("ldconsole.exe");
    if direct_ld.exists() {
        return Ok(direct_ld);
    }
    let sub9 = dir.join("LDPlayer9").join("ldconsole.exe");
    if sub9.exists() {
        return Ok(sub9);
    }
    let sub4 = dir.join("LDPlayer4").join("ldconsole.exe");
    if sub4.exists() {
        return Ok(sub4);
    }
    Err("ldconsole.exe not found in specified directory".into())
}

pub async fn run_ldconsole_cmd(ldplayer_dir: &str, args: &[&str]) -> Result<String, String> {
    let exe = get_executable_path(ldplayer_dir)?;
    let parent = exe.parent().unwrap_or_else(|| Path::new(ldplayer_dir));
    let output = Command::new(&exe)
        .current_dir(parent)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute ldconsole: {}", e))?;

    let out_str = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(out_str)
}

#[tauri::command]
pub fn auto_detect_ldplayer() -> Option<String> {
    let candidates = [
        r"C:\LDPlayer\LDPlayer9",
        r"D:\LDPlayer\LDPlayer9",
        r"E:\LDPlayer\LDPlayer9",
        r"C:\leidian\LDPlayer9",
        r"D:\leidian\LDPlayer9",
        r"C:\LDPlayer\LDPlayer4.0",
        r"D:\LDPlayer\LDPlayer4.0",
    ];

    for path in &candidates {
        if Path::new(path).join("ldconsole.exe").exists() {
            return Some(path.to_string());
        }
    }
    None
}

fn is_ldplayer_running(parts: &[&str]) -> bool {
    let process_alive = |pos: usize| {
        parts
            .get(pos)
            .and_then(|v| v.trim().parse::<i32>().ok())
            .map(|pid| pid > 0)
            .unwrap_or(false)
    };

    let hwnd_alive = |pos: usize| {
        parts
            .get(pos)
            .and_then(|v| v.trim().parse::<i64>().ok())
            .map(|hwnd| hwnd > 0)
            .unwrap_or(false)
    };

    parts
        .get(4)
        .map(|v| v.trim() == "1" || v.trim() == "true")
        .unwrap_or(false)
        || process_alive(5)
        || process_alive(6)
        || hwnd_alive(2)
        || hwnd_alive(3)
}

#[tauri::command]
pub async fn list_emulators(ldplayer_dir: String) -> Result<Vec<Emulator>, String> {
    let raw = run_ldconsole_cmd(&ldplayer_dir, &["list2"]).await?;
    let mut emulators = Vec::new();

    for line in raw.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 {
            continue;
        }

        let index: i32 = parts[0].trim().parse().unwrap_or(-1);
        if index < 0 {
            continue;
        }
        let name = parts[1].trim().to_string();
        let top_hwnd = parts[2].trim().to_string();
        let bind_hwnd = parts[3].trim().to_string();
        let is_running = is_ldplayer_running(&parts);
        let pid: i32 = parts[5].trim().parse().unwrap_or(-1);
        let vbox_pid: i32 = parts[6].trim().parse().unwrap_or(-1);

        let width = parts.get(7).and_then(|v| v.trim().parse().ok()).unwrap_or(720);
        let height = parts.get(8).and_then(|v| v.trim().parse().ok()).unwrap_or(1280);
        let dpi = parts.get(9).and_then(|v| v.trim().parse().ok()).unwrap_or(320);

        emulators.push(Emulator {
            index,
            name,
            top_hwnd,
            bind_hwnd,
            is_running,
            pid,
            vbox_pid,
            width,
            height,
            dpi,
            brand: "Samsung".into(),
            model: "Galaxy S22".into(),
            imei: "".into(),
            mac: "".into(),
            android_id: "".into(),
            disk_size_bytes: 0,
        });
    }

    Ok(emulators)
}

#[tauri::command]
pub async fn launch_emulator(ldplayer_dir: String, index: i32) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["launch", "--index", &index.to_string()]).await?;
    Ok(())
}

#[tauri::command]
pub async fn quit_emulator(ldplayer_dir: String, index: i32) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["quit", "--index", &index.to_string()]).await?;
    Ok(())
}

#[tauri::command]
pub async fn quit_all_emulators(ldplayer_dir: String) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["quitall"]).await?;
    Ok(())
}

fn apply_default_instance_settings(
    ldplayer_dir: &str,
    index: i32,
    remember_wnd: bool,
    auto_rotate: bool,
    lock_window: bool,
    system_disk_writable: bool,
) -> Result<(), String> {
    let config_path = std::path::Path::new(ldplayer_dir)
        .join("vms")
        .join("config")
        .join(format!("leidian{}.config", index));

    if !config_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut json_val: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file JSON: {}", e))?;

    if let Some(obj) = json_val.as_object_mut() {
        obj.insert("basicSettings.adbDebug".to_string(), serde_json::Value::from(1));
        obj.insert("basicSettings.rootMode".to_string(), serde_json::Value::from(true));
        obj.insert("basicSettings.rememberWndPos".to_string(), serde_json::Value::from(remember_wnd));
        obj.insert("basicSettings.autoRotate".to_string(), serde_json::Value::from(auto_rotate));
        obj.insert("basicSettings.lockWindowSize".to_string(), serde_json::Value::from(lock_window));
        obj.insert("advancedSettings.systemDiskMode".to_string(), serde_json::Value::from(if system_disk_writable { 1 } else { 0 }));

        let updated = serde_json::to_string_pretty(&json_val)
            .map_err(|e| format!("Failed to serialize config JSON: {}", e))?;
        let _ = std::fs::write(&config_path, updated);
    }
    Ok(())
}

#[tauri::command]
pub async fn add_emulator(
    ldplayer_dir: String,
    name: Option<String>,
    remember_wnd: Option<bool>,
    auto_rotate: Option<bool>,
    lock_window: Option<bool>,
    system_disk_writable: Option<bool>,
) -> Result<i32, String> {
    let mut args = vec!["add"];
    let name_str;
    if let Some(ref n) = name {
        if !n.trim().is_empty() {
            args.push("--name");
            name_str = n.trim().to_string();
            args.push(&name_str);
        }
    }
    let raw = run_ldconsole_cmd(&ldplayer_dir, &args).await?;
    let index: i32 = raw.trim().parse().unwrap_or(-1);

    if index >= 0 {
        let _ = apply_default_instance_settings(
            &ldplayer_dir,
            index,
            remember_wnd.unwrap_or(true),
            auto_rotate.unwrap_or(false),
            lock_window.unwrap_or(true),
            system_disk_writable.unwrap_or(true),
        );
    }

    Ok(index)
}

#[tauri::command]
pub async fn copy_emulator(
    ldplayer_dir: String,
    name: String,
    from_index: i32,
) -> Result<(), String> {
    run_ldconsole_cmd(
        &ldplayer_dir,
        &["copy", "--name", &name, "--from", &from_index.to_string()],
    )
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_emulator(ldplayer_dir: String, index: i32) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["remove", "--index", &index.to_string()]).await?;
    Ok(())
}

#[tauri::command]
pub async fn rename_emulator(ldplayer_dir: String, index: i32, title: String) -> Result<(), String> {
    run_ldconsole_cmd(
        &ldplayer_dir,
        &["rename", "--index", &index.to_string(), "--title", &title],
    )
    .await?;
    Ok(())
}

fn parse_hwnd(value: &str) -> Result<isize, String> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        isize::from_str_radix(hex, 16).map_err(|_| format!("Invalid window handle: {value}"))
    } else {
        trimmed
            .parse::<isize>()
            .map_err(|_| format!("Invalid window handle: {value}"))
    }
}

#[cfg(target_os = "windows")]
fn fit_arranged_window_size(
    max_window_width: i32,
    max_window_height: i32,
    horizontal_chrome: i32,
    vertical_chrome: i32,
    resolution_width: i32,
    resolution_height: i32,
) -> (i32, i32) {
    let max_window_width = max_window_width.max(1);
    let max_window_height = max_window_height.max(1);
    let horizontal_chrome = horizontal_chrome.clamp(0, max_window_width - 1);
    let vertical_chrome = vertical_chrome.clamp(0, max_window_height - 1);
    let resolution_width = resolution_width.max(1);
    let resolution_height = resolution_height.max(1);
    let max_content_width = (max_window_width - horizontal_chrome).max(1);
    let max_content_height = (max_window_height - vertical_chrome).max(1);

    let scale_dimension = |value: i32, numerator: i32, denominator: i32| {
        ((i64::from(value) * i64::from(numerator)) / i64::from(denominator))
            .clamp(1, i64::from(i32::MAX)) as i32
    };

    let mut content_width = max_content_width;
    let mut content_height = scale_dimension(content_width, resolution_height, resolution_width);
    if content_height > max_content_height {
        content_height = max_content_height;
        content_width = scale_dimension(content_height, resolution_width, resolution_height)
            .min(max_content_width);
    }

    (
        (content_width + horizontal_chrome).min(max_window_width),
        (content_height + vertical_chrome).min(max_window_height),
    )
}

#[cfg(target_os = "windows")]
fn arrange_windows_win32(
    hwnds: &[String],
    bind_hwnds: &[String],
    resolution_widths: &[i32],
    resolution_heights: &[i32],
    cols: i32,
) -> Result<(), String> {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClientRect, GetSystemMetrics, GetWindowRect, IsWindow, SetWindowPos, ShowWindow,
        SystemParametersInfoW, SM_CXSCREEN, SM_CYSCREEN, SPI_GETWORKAREA, SWP_NOACTIVATE,
        SWP_NOZORDER, SW_RESTORE,
    };

    struct ArrangeTarget {
        hwnd: isize,
        bind_hwnd: Option<isize>,
        resolution_width: i32,
        resolution_height: i32,
    }

    let columns = cols.max(1);
    let mut seen = std::collections::HashSet::new();
    let valid_targets = hwnds
        .iter()
        .enumerate()
        .filter_map(|(index, raw_hwnd)| {
            let hwnd = parse_hwnd(raw_hwnd).ok()?;
            if unsafe { IsWindow(hwnd) } == 0 || !seen.insert(hwnd) {
                return None;
            }
            let bind_hwnd = bind_hwnds
                .get(index)
                .and_then(|raw_bind_hwnd| parse_hwnd(raw_bind_hwnd).ok())
                .filter(|bind_hwnd| unsafe { IsWindow(*bind_hwnd) } != 0);
            Some(ArrangeTarget {
                hwnd,
                bind_hwnd,
                resolution_width: resolution_widths.get(index).copied().unwrap_or(0),
                resolution_height: resolution_heights.get(index).copied().unwrap_or(0),
            })
        })
        .collect::<Vec<_>>();

    if valid_targets.is_empty() {
        return Err("No valid emulator windows are ready to arrange.".to_string());
    }

    let mut work_area = RECT {
        left: 0,
        top: 0,
        right: unsafe { GetSystemMetrics(SM_CXSCREEN) }.max(1),
        bottom: unsafe { GetSystemMetrics(SM_CYSCREEN) }.max(1),
    };
    unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            &mut work_area as *mut RECT as *mut core::ffi::c_void,
            0,
        );
    }

    let work_width = (work_area.right - work_area.left).max(1);
    let work_height = (work_area.bottom - work_area.top).max(1);
    let rows = ((valid_targets.len() as i32 + columns - 1) / columns).max(1);
    let gap = 1;
    let mut arranged_count = 0;
    let mut last_error = None;

    for (position, target) in valid_targets.into_iter().enumerate() {
        let column = position as i32 % columns;
        let row = position as i32 / columns;
        let cell_left = work_area.left + column * work_width / columns;
        let cell_right = work_area.left + (column + 1) * work_width / columns;
        let cell_top = work_area.top + row * work_height / rows;
        let cell_bottom = work_area.top + (row + 1) * work_height / rows;
        let max_window_width = (cell_right - cell_left - gap).max(1);
        let max_window_height = (cell_bottom - cell_top - gap).max(1);

        let mut horizontal_chrome = 0;
        let mut vertical_chrome = 0;
        let mut current_content_width = 0;
        let mut current_content_height = 0;
        if let Some(bind_hwnd) = target.bind_hwnd {
            let mut window_rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            let mut client_rect = window_rect;
            let mut bind_rect = window_rect;
            let has_window_rect = unsafe { GetWindowRect(target.hwnd, &mut window_rect) } != 0;
            let has_client_rect = unsafe { GetClientRect(target.hwnd, &mut client_rect) } != 0;
            let has_bind_rect = unsafe { GetWindowRect(bind_hwnd, &mut bind_rect) } != 0;

            if has_window_rect && has_client_rect && has_bind_rect {
                let outer_width = (window_rect.right - window_rect.left).max(1);
                let outer_height = (window_rect.bottom - window_rect.top).max(1);
                let client_width = (client_rect.right - client_rect.left).max(1);
                let client_height = (client_rect.bottom - client_rect.top).max(1);
                current_content_width = (bind_rect.right - bind_rect.left).max(1);
                current_content_height = (bind_rect.bottom - bind_rect.top).max(1);

                horizontal_chrome = (outer_width - current_content_width).max(0);
                let toolbar_width = (client_width - current_content_width).max(0);
                let native_vertical_border = (outer_height - client_height).max(0);
                vertical_chrome = toolbar_width + native_vertical_border;
            }
        }

        let mut resolution_width = target.resolution_width;
        let mut resolution_height = target.resolution_height;
        if resolution_width <= 0 || resolution_height <= 0 {
            resolution_width = current_content_width;
            resolution_height = current_content_height;
        }
        if resolution_width <= 0 || resolution_height <= 0 {
            resolution_width = 9;
            resolution_height = 16;
        }
        let configured_landscape = resolution_width > resolution_height;
        let current_landscape = current_content_width > current_content_height;
        if current_content_width > 0
            && current_content_height > 0
            && configured_landscape != current_landscape
        {
            std::mem::swap(&mut resolution_width, &mut resolution_height);
        }

        let (width, height) = fit_arranged_window_size(
            max_window_width,
            max_window_height,
            horizontal_chrome,
            vertical_chrome,
            resolution_width,
            resolution_height,
        );
        let x = cell_left + ((cell_right - cell_left - width) / 2).max(0);

        unsafe {
            ShowWindow(target.hwnd, SW_RESTORE);
            if SetWindowPos(
                target.hwnd,
                0,
                x,
                cell_top,
                width,
                height,
                SWP_NOACTIVATE | SWP_NOZORDER,
            ) != 0
            {
                arranged_count += 1;
            } else {
                last_error = Some(std::io::Error::last_os_error().to_string());
            }
        }
    }

    if arranged_count == 0 {
        return Err(format!(
            "Windows could not be arranged: {}",
            last_error.unwrap_or_else(|| "unknown Windows API error".to_string())
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn sort_windows(
    ldplayer_dir: String,
    hwnds: Option<Vec<String>>,
    bind_hwnds: Option<Vec<String>>,
    resolution_widths: Option<Vec<i32>>,
    resolution_heights: Option<Vec<i32>>,
    cols: Option<i32>,
) -> Result<(), String> {
    if let (Some(h), Some(b), Some(rw), Some(rh)) = (hwnds, bind_hwnds, resolution_widths, resolution_heights) {
        #[cfg(target_os = "windows")]
        {
            arrange_windows_win32(&h, &b, &rw, &rh, cols.unwrap_or(4))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (h, b, rw, rh, cols);
            run_ldconsole_cmd(&ldplayer_dir, &["sortWnd"]).await?;
            Ok(())
        }
    } else {
        run_ldconsole_cmd(&ldplayer_dir, &["sortWnd"]).await?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn collect_matching_windows(hwnd: isize, context: isize) -> i32 {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetWindowTextLengthW, GetWindowTextW,
    };

    let request = &mut *(context as *mut (Vec<String>, Vec<isize>));

    // Check Class Name
    let mut class_buffer = [0u16; 256];
    let class_len = GetClassNameW(hwnd, class_buffer.as_mut_ptr(), class_buffer.len() as i32);
    let class_name = if class_len > 0 {
        String::from_utf16_lossy(&class_buffer[..class_len as usize])
    } else {
        String::new()
    };

    // Check Window Title
    let title_length = GetWindowTextLengthW(hwnd);
    let title = if title_length > 0 {
        let mut title_buffer = vec![0u16; title_length as usize + 1];
        let copied = GetWindowTextW(hwnd, title_buffer.as_mut_ptr(), title_buffer.len() as i32);
        if copied > 0 {
            String::from_utf16_lossy(&title_buffer[..copied as usize])
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let is_ldplayer_class = class_name.contains("LDPlayer")
        || class_name.contains("RenderWindow")
        || class_name.contains("subWin");

    let is_matching_title = request.0.iter().any(|name| {
        if name.is_empty() {
            return false;
        }
        title.contains(name)
            || title.starts_with("LDPlayer")
            || name.starts_with(&title)
    }) || title.starts_with("LDPlayer");

    if (is_matching_title || is_ldplayer_class) && !request.1.contains(&hwnd) {
        request.1.push(hwnd);
    }
    1
}

#[tauri::command]
pub async fn toggle_emulators_visibility(
    running_names: Option<Vec<String>>,
    hwnds: Option<Vec<String>>,
) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            EnumWindows, IsWindow, IsWindowVisible, ShowWindow, SW_HIDE, SW_RESTORE,
        };

        let mut target_hwnds: Vec<isize> = Vec::new();

        // 1. Direct HWND lookup
        if let Some(handles) = hwnds {
            for raw in handles {
                if let Ok(hwnd) = parse_hwnd(&raw) {
                    if unsafe { IsWindow(hwnd) } != 0 && !target_hwnds.contains(&hwnd) {
                        target_hwnds.push(hwnd);
                    }
                }
            }
        }

        // 2. Enumerated windows by name/class
        let names = running_names.unwrap_or_default();
        let mut request = (names, Vec::<isize>::new());
        unsafe {
            EnumWindows(
                Some(collect_matching_windows),
                &mut request as *mut (Vec<String>, Vec<isize>) as isize,
            );
        }

        for h in request.1 {
            if !target_hwnds.contains(&h) {
                target_hwnds.push(h);
            }
        }

        if target_hwnds.is_empty() {
            return Err("No running emulator windows were found".to_string());
        }

        let should_hide = target_hwnds
            .iter()
            .any(|&hwnd| unsafe { IsWindowVisible(hwnd) } != 0);

        for hwnd in target_hwnds {
            unsafe {
                ShowWindow(hwnd, if should_hide { SW_HIDE } else { SW_RESTORE });
            }
        }
        Ok(should_hide)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (running_names, hwnds);
        Err("Window visibility controls are only supported on Windows".to_string())
    }
}

#[tauri::command]
pub async fn modify_emulator(
    ldplayer_dir: String,
    index: i32,
    resolution: Option<String>,
    cpu: Option<i32>,
    memory: Option<i32>,
    root: Option<String>,
) -> Result<(), String> {
    let index_str = index.to_string();
    let mut args = vec!["modify", "--index", &index_str];
    let cpu_str = cpu.map(|c| c.to_string());
    let mem_str = memory.map(|m| m.to_string());

    if let Some(ref r) = resolution {
        args.push("--resolution");
        args.push(r);
    }
    if let Some(ref c) = cpu_str {
        args.push("--cpu");
        args.push(c);
    }
    if let Some(ref m) = mem_str {
        args.push("--memory");
        args.push(m);
    }
    if let Some(ref root_val) = root {
        args.push("--root");
        args.push(root_val);
    }

    let _ = run_ldconsole_cmd(&ldplayer_dir, &args).await;

    // Directly ensure leidian<index>.config has exact resolution values
    let config_path = std::path::Path::new(&ldplayer_dir)
        .join("vms")
        .join("config")
        .join(format!("leidian{}.config", index));

    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(mut json_val) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(obj) = json_val.as_object_mut() {
                    if let Some(ref r) = resolution {
                        let parts: Vec<&str> = r.split(',').collect();
                        if parts.len() == 3 {
                            let w: i32 = parts[0].trim().parse().unwrap_or(540);
                            let h: i32 = parts[1].trim().parse().unwrap_or(960);
                            let dpi: i32 = parts[2].trim().parse().unwrap_or(240);

                            obj.insert(
                                "advancedSettings.resolution".to_string(),
                                serde_json::json!({ "width": w, "height": h }),
                            );
                            obj.insert(
                                "advancedSettings.resolutionDpi".to_string(),
                                serde_json::Value::from(dpi),
                            );
                            for key in [
                                "basicSettings.width",
                                "basicSettings.height",
                                "basicSettings.realWidth",
                                "basicSettings.realHeigh",
                            ] {
                                obj.insert(key.to_string(), serde_json::Value::from(-1));
                            }
                        }
                    }
                    if let Some(c) = cpu {
                        obj.insert("basicSettings.cpuCount".to_string(), serde_json::Value::from(c));
                    }
                    if let Some(m) = memory {
                        obj.insert("basicSettings.memorySize".to_string(), serde_json::Value::from(m));
                    }
                    if let Some(ref root_val) = root {
                        obj.insert("basicSettings.rootMode".to_string(), serde_json::Value::from(root_val == "1"));
                    }

                    if let Ok(updated) = serde_json::to_string_pretty(&json_val) {
                        let _ = std::fs::write(&config_path, updated);
                    }
                }
            }
        }
    }

    Ok(())
}
