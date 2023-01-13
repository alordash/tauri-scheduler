#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::connection::establish_connection_pool;
use services::tasks_service::tasks_service::get_all_tasks;

pub mod db;
pub mod model;
pub mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connections = establish_connection_pool().await.unwrap();
    tauri::Builder::default()
        .manage(connections) // Makes connection pool available in all #[tauri::command]
        .invoke_handler(tauri::generate_handler![get_all_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
