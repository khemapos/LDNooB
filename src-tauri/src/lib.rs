pub mod domains;

use domains::automation::*;
use domains::database::*;
use domains::emulator::*;
use domains::proxy::*;
use domains::window_ops::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to LDNooB!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Err(e) = init_db(&app.handle().clone()) {
                eprintln!("Warning: Failed to initialize SQLite database: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Database
            db_get,
            db_set,
            db_delete,
            db_get_all,
            // Emulator Fleet
            auto_detect_ldplayer,
            list_emulators,
            launch_emulator,
            quit_emulator,
            quit_all_emulators,
            add_emulator,
            copy_emulator,
            remove_emulator,
            rename_emulator,
            sort_windows,
            modify_emulator,
            // Proxy
            check_proxy,
            get_host_ip,
            // Automation & ADB
            read_binary_file,
            run_adb_command,
            start_app,
            human_swipe,
            human_type,
            // Window Operations
            app_toggle_maximize,
            app_minimize,
            app_close,
            app_is_maximized,
            app_set_window_size,
            app_get_window_size,
            focus_emulator_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
