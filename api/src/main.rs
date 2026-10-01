mod config;
mod db;
mod errors;
mod models;
mod routes;

use crate::config::get_database_url;
use crate::db::get_pool;
use crate::routes::routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = get_database_url();
    let pool = get_pool(&database_url).await;

    let app = routes(pool);

    let addr = "0.0.0.0:8000";
    println!("API running on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
