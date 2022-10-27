#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod model;
pub mod sm2;
pub mod utils;

#[tokio::main]
async fn main() {
    utils::init_tracing();

    tauri::Builder::default()
        .manage(model::DbConnection::default().await)
        .invoke_handler(tauri::generate_handler![
            model::get_task_by_id,
            model::add_task,
            model::get_tasks,
            model::review_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
