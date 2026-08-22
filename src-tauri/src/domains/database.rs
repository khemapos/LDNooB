use rusqlite::Connection;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Manager;

pub fn get_db_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {}", e))?;
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create database directory: {}", e))?;
    path.push("ldnoob.db");
    Ok(path)
}

pub fn get_db_conn(app: &tauri::AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app)?;
    Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))
}

pub fn init_db(app: &tauri::AppHandle) -> Result<(), String> {
    let conn = get_db_conn(app)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        )",
        [],
    )
    .map_err(|e| format!("Failed to create settings table: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn db_get(app: tauri::AppHandle, key: String) -> Result<Option<String>, String> {
    let conn = get_db_conn(&app)?;
    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = ?")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt.query([&key]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let val: String = row.get(0).map_err(|e| e.to_string())?;
        Ok(Some(val))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn db_set(app: tauri::AppHandle, key: String, value: String) -> Result<(), String> {
    let conn = get_db_conn(&app)?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
        [&key, &value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn db_delete(app: tauri::AppHandle, key: String) -> Result<(), String> {
    let conn = get_db_conn(&app)?;
    conn.execute("DELETE FROM settings WHERE key = ?", [&key])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn db_get_all(app: tauri::AppHandle) -> Result<HashMap<String, String>, String> {
    let conn = get_db_conn(&app)?;
    let mut stmt = conn
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            let k: String = row.get(0)?;
            let v: String = row.get(1)?;
            Ok((k, v))
        })
        .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for row in rows {
        if let Ok((k, v)) = row {
            map.insert(k, v);
        }
    }
    Ok(map)
}
