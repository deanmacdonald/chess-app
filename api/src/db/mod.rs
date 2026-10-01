use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn get_pool(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database")
}
