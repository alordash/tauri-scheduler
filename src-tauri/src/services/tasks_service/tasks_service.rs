use sqlx::PgPool;
use tauri::State;

use crate::{
    db::{connection::DbConnectionPool, tasks_controller},
    model::task::task::Task,
};

#[tauri::command]
pub async fn get_all_tasks<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Task>, String> {
    let pool = &*connection.connection.lock().await;
    let tasks = tasks_controller::get_all_tasks(pool)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(tasks)
}
