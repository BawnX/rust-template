use actix::SyncArbiter;
use crate::config::database::{DbExecutor, setup};
use crate::modules::shared::model::{AppState, HealthActor};

pub mod repository;
pub mod model;

pub async fn init_state() -> AppState {
    let health_actor_addr_start = SyncArbiter::start(1, move || HealthActor);
    let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = setup(conn_str).await;
    let db_actor_addr_start = SyncArbiter::start(1, move || DbExecutor(db.clone()));
    AppState {
        health_actor_addr: health_actor_addr_start,
        db_actor_addr: db_actor_addr_start
    }
}