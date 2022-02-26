use crate::config::models::CqrsResult;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Command<DATA, CREATE, RESULT>: Send + Sync + 'static {
    fn new(&mut self, data: DATA) -> CREATE;

    async fn execute(&self) -> CqrsResult<RESULT>;
}
