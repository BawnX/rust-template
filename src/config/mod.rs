use crate::config::dotenv::dotenv;
use crate::config::tracing::{log, tracing};

mod dotenv;
pub mod models;
pub mod repository;
pub mod thread;
pub mod tracing;
pub mod cqrs;

pub fn initialize() -> String {
    dotenv();
    tracing();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);

    log::debug!("Starting our Server at: {}", address);

    return address;
}
