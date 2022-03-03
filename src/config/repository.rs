use std::future::{ready, Ready};
use std::ops::Deref;
use std::sync::Arc;

use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use actix_web::{Error, FromRequest, HttpRequest};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Repository<PARAM: 'static, OBJ: 'static>: Send + Sync + 'static {}

pub struct RepositoryInjector<PARAM: 'static, OBJ: 'static>(Arc<Box<dyn Repository<PARAM, OBJ>>>);

impl<PARAM: 'static, OBJ: 'static> Clone for RepositoryInjector<PARAM, OBJ> {
    fn clone(&self) -> Self {
        let repo = self.0.clone();
        Self(repo)
    }
}

impl<PARAM: 'static, OBJ: 'static> Deref for RepositoryInjector<PARAM, OBJ> {
    type Target = dyn Repository<PARAM, OBJ>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().as_ref()
    }
}

impl<PARAM: 'static, OBJ: 'static> FromRequest for RepositoryInjector<PARAM, OBJ> {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(injector) = req.app_data::<Self>() {
            ready(Ok(injector.to_owned()))
        } else {
            ready(Err(ErrorBadRequest(
                "Not repository injector was found in the request",
            )))
        }
    }
}
