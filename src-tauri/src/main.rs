#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod model;
pub mod sm2;
pub mod utils;

fn main() {
    utils::init_tracing();

    tauri::Builder::default()
        .manage(model::DbConnection::default())
        .invoke_handler(tauri::generate_handler![
            model::add_task,
            model::get_tasks,
            model::review_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
