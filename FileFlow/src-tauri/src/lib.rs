// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri::command]
fn set_dark_mode(enabled: bool) -> String {
    if enabled {
        String::from("Dark mode is on")
    } else {
        String::from("Dark mode is off")
    }
}

#[tauri::command]
fn count_files(file_names: Vec<String>) -> usize {
    file_names.len()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_numbers,
            set_dark_mode,
            count_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
