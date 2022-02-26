use crate::config::cqrs::query::Query;
use crate::config::models::CqrsResult;
use crate::config::tracing::log;
use crate::modules::health::models::{Health, ObtainHealthQuery};
use async_trait::async_trait;
use std::sync::RwLock;

#[async_trait]
impl Query<bool, ObtainHealthQuery, Health> for ObtainHealthQuery {
    fn new(&mut self, data: bool) -> ObtainHealthQuery {
        ObtainHealthQuery {
            system_status: RwLock::from(data),
        }
    }

    async fn execute(&self) -> CqrsResult<Health> {
        let system_status = self.system_status.read()?;
        let system_status = *system_status;
        let health = Health { system_status };
        Ok(health)
    }
}
