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

#[tauri::command]
pub async fn get_user_tasks<'r>(
    user_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Task>, String> {
    let pool = &*connection.connection.lock().await;
    let tasks = tasks_controller::get_user_tasks(pool, user_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(tasks)
}

#[tauri::command]
pub async fn add_task<'r>(
    task: Task,
    connection: State<'r, DbConnectionPool>,
) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;
    let task_id = tasks_controller::add_task(pool, task)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(task_id)
}

#[tauri::command]
pub async fn complete_task<'r>(
    task_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = tasks_controller::complete_task(pool, task_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn remove_task<'r>(
    task_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = tasks_controller::remove_task(pool, task_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(rows_affected)
}
