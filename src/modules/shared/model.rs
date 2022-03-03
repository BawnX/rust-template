use actix::{Actor, Addr, SyncContext};
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
}

#[derive(Cherry)]
#[cherry(table = "[Sample].[UserSample]")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub description: String,
}
