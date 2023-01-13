#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
async fn test_db() -> i32 {
    use sqlx::postgres::PgPoolOptions;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await
        .unwrap();

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i32,) = sqlx::query_as("SELECT $1")
        .bind(150_i32)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("row.0: {}", row.0);
    
    row.0
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
