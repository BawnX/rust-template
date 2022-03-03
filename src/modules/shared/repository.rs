use std::error::Error;
use cherry::DataSource;
use crate::config::database::DbExecutor;
use crate::modules::shared::model::User;

pub async fn select_user() -> Result<Option<User>, Box<dyn Error>>  {
    let select: Option<User> = DbExecutor.select().fetch().await?;
    Ok(select)
}