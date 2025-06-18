use sqlx::{self, Pool, Postgres};

pub async fn create_tables(pool: &Pool<Postgres>) {
    sqlx::query(include_str!("./queries/ct_users.sql"))
        .execute(pool)
        .await
        .unwrap();
}