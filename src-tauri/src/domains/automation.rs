use crate::domains::emulator::run_ldconsole_cmd;

#[tauri::command]
pub fn read_binary_file(file_path: String) -> Result<Vec<u8>, String> {
    let path = std::path::Path::new(&file_path);
    std::fs::read(path).map_err(|e| format!("Failed to read binary file: {}", e))
}

#[tauri::command]
pub async fn run_adb_command(
    ldplayer_dir: String,
    index: i32,
    adb_command: String,
) -> Result<String, String> {
    let index_str = index.to_string();
    let args = vec!["adb", "--index", &index_str, "--command", &adb_command];
    run_ldconsole_cmd(&ldplayer_dir, &args).await
}

#[tauri::command]
pub async fn start_app(
    ldplayer_dir: String,
    index: i32,
    package_name: String,
) -> Result<(), String> {
    let index_str = index.to_string();
    let args = vec!["runapp", "--index", &index_str, "--packagename", &package_name];
    run_ldconsole_cmd(&ldplayer_dir, &args).await?;
    Ok(())
}

#[tauri::command]
pub async fn human_swipe(
    ldplayer_dir: String,
    index: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    duration_ms: Option<u64>,
) -> Result<(), String> {
    let duration = duration_ms.unwrap_or(300);
    let cmd = format!("input swipe {} {} {} {} {}", x1, y1, x2, y2, duration);
    run_adb_command(ldplayer_dir, index, cmd).await?;
    Ok(())
}

#[tauri::command]
pub async fn human_type(
    ldplayer_dir: String,
    index: i32,
    text: String,
    _with_typo: Option<bool>,
) -> Result<(), String> {
    let sanitized = text.replace(' ', "%s");
    let cmd = format!("input text {}", sanitized);
    run_adb_command(ldplayer_dir, index, cmd).await?;
    Ok(())
}
