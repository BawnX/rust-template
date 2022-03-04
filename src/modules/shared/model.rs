use actix::{Actor, Addr, SyncContext};
use crate::config::database::DbExecutor;
use crate::config::tracing::log;

#[derive(Debug)]
pub struct HealthActor;

impl Actor for HealthActor{
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        log::trace!("started Health Actor");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        log::trace!("stopped Health Actor");
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub health_actor_addr: Addr<HealthActor>,
    pub db_actor_addr: Addr<DbExecutor>,
}

pub struct User {
    pub id: i32,
    pub name: String,
    pub description: String,
}
