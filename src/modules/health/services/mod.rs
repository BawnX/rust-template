use actix::{Message, Handler};
use futures::executor;
use crate::modules::health::models::{Health, HealthResponse, ObtainHealth};
use crate::config::models::{RepositoryResult};
use crate::config::models::enums::RepositoryError;
use crate::modules::shared::model::HealthActor;
use crate::modules::shared::repository;

impl Message for ObtainHealth {
    type Result = RepositoryResult<HealthResponse>;
}

impl Handler<ObtainHealth> for HealthActor {
    type Result = RepositoryResult<HealthResponse>;

    fn handle(&mut self, msg: ObtainHealth, _: &mut Self::Context) -> Self::Result {
        let mut health = HealthResponse {
            health: Health {
                database_status: msg.database_status,
                system_status: msg.system_status,
            }
        };

        let values = executor::block_on(repository::select_user()).unwrap();

        match values {
            Some(_) => {
                health.health.database_status = true;
                Ok(health)
            },
            None => Err(RepositoryError::LockError("Database dose-nt work".into()))
        }
    }
}
