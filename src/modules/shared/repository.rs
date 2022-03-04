use std::error::Error;
use std::sync::Arc;
use actix::{Handler, Message};
use futures::executor;
use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;
use crate::config::database::DbExecutor;
use crate::config::models::enums::RepositoryError;
use crate::config::models::RepositoryResult;
use crate::config::tracing::log;
use crate::modules::health::models::{Health, HealthResponse, ObtainHealth};

impl Message for ObtainHealth {
    type Result = RepositoryResult<HealthResponse>;
}

impl Handler<ObtainHealth> for DbExecutor {
    type Result = RepositoryResult<HealthResponse>;

    fn handle(&mut self, msg: ObtainHealth, _: &mut Self::Context) -> Self::Result {
        let mut health = HealthResponse {
            health: Health {
                database_status: msg.database_status,
                system_status: msg.system_status,
            }
        };

        let values = executor::block_on(select_user(self.0.clone())).unwrap();

        match values {
            Some(val) => {
                log::info!("Count data {}", val);
                health.health.database_status = true;
                Ok(health)
            },
            None => Err(RepositoryError::LockError("Database dose-nt work".into()))
        }
    }
}

pub async fn select_user(arc: Arc<Rbatis>) -> Result<Option<i32>, Box<dyn Error>>  {
    let select:Option<i32> = arc.fetch("select count(*) from [Sample].[UserSample]", vec![]).await.unwrap();
    Ok(select)
}