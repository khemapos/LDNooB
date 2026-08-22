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
        let is_running = parts[4].trim() == "1";
        let pid: i32 = parts[5].trim().parse().unwrap_or(-1);
        let vbox_pid: i32 = parts[6].trim().parse().unwrap_or(-1);

        emulators.push(Emulator {
            index,
            name,
            top_hwnd,
            bind_hwnd,
            is_running,
            pid,
            vbox_pid,
            width: 720,
            height: 1280,
            dpi: 320,
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

#[tauri::command]
pub async fn add_emulator(ldplayer_dir: String, name: String) -> Result<(), String> {
    run_ldconsole_cmd(&ldplayer_dir, &["add", &name]).await?;
    Ok(())
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

    run_ldconsole_cmd(&ldplayer_dir, &args).await?;
    Ok(())
}
