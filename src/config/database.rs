use std::any::Any;
use std::error::Error;
use cherry::connection::{PoolConfig, setup_pools};
use cherry::DataSource;

pub struct DbExecutor;

impl DataSource for DbExecutor {}

pub async fn setup(url: String) -> Result<DbExecutor, Box<dyn Error>> {
    let config = [
        (DbExecutor.type_id(), PoolConfig {
            url: url.to_owned(),
            ..Default::default()
        }),
    ];
    setup_pools(config).await?;
    Ok(DbExecutor)
}