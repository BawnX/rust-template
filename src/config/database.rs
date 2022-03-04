use std::sync::Arc;
use actix::{Actor, SyncContext};
use rbatis::rbatis::Rbatis;
use crate::config::tracing::log;

#[derive(Debug, Clone)]
pub struct DbExecutor(pub  Arc<Rbatis>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub async fn setup(url: String) -> Arc<Rbatis> {
    let rb = Rbatis::new();
    rb.link(&*url).await.expect("rbatis link database fail");
    log::info!("linking database successful!");
    return Arc::new(rb);
}