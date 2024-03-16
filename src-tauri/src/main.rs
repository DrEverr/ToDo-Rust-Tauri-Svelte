// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize, serde::Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

static mut COUNTER: i32 = 0;

#[tauri::command]
fn generate_todo(title: &str) -> Todo {
    unsafe {
        COUNTER += 1;
        Todo {
            id: COUNTER,
            title: title.to_string(),
            completed: false,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
