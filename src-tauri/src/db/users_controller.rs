use sqlx::PgPool;

use crate::model::user::{dao::user_entity::UserEntity, user::User};

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let user_entities = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT *
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(user_entities.into_iter().map(User::from).collect())
}

pub async fn add_user(pool: &PgPool, user: User) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (login, password)
        VALUES ( $1, crypt($2, gen_salt('bf')) )
        RETURNING id
        "#,
        user.login(),
        user.password()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn remove_user(pool: &PgPool, user_id: i64) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}
