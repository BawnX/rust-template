// use actix::{Addr, SyncArbiter};
use crate::config::database::setup;
use crate::config::dotenv::dotenv;
use crate::config::tracing::{log, tracing};

mod dotenv;
pub mod models;
pub mod repository;
pub mod thread;
pub mod tracing;
pub mod database;

pub async fn initialize() -> String {
    dotenv();
    tracing();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _ = setup(conn_str).await.unwrap();

    log::debug!("Starting our Server at: {}", address);
    address
}