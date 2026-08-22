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

#[tauri::command]
pub async fn sort_windows(ldplayer_dir: String) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["sortWnd"]).await?;
    Ok(())
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
