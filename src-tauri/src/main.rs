#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;
pub mod model;
pub mod services;

use db::connection::establish_connection_pool;
use model::task::task::new_task;
use services::tasks_service::tasks_service::*;
use tauri::Manager;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connections = establish_connection_pool().await.unwrap();
    tauri::Builder::default()
        .manage(connections) // Makes connection pool available in all #[tauri::command]
        .invoke_handler(tauri::generate_handler![
            new_task,
            get_all_tasks,
            get_user_tasks,
            add_task,
            complete_task,
            remove_task
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
