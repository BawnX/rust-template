use actix::SyncArbiter;
use crate::modules::shared::model::{AppState, HealthActor};

pub mod repository;
pub mod model;

pub fn init_state() -> AppState {
    let health_actor_addr_start = SyncArbiter::start(num_cpus::get(), move || HealthActor);
    AppState {
        health_actor_addr: health_actor_addr_start,
    }
}