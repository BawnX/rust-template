use crate::modules::health::models::{ObtainHealth};
use actix_web::web::Data;
use actix_web::HttpResponse;
use tracing::instrument;
use crate::config::models::ErrorResponse;
use crate::modules::shared::model::AppState;

#[instrument]
pub async fn get_health_check(index: Data<u16>, app: Data<AppState>) -> HttpResponse {
    let obtain_health = ObtainHealth { database_status: false, system_status: true };

    match app.health_actor_addr.send(obtain_health).await.unwrap() {
        Ok(res) => HttpResponse::Ok()
            .append_header(("thread-id", index.to_string()))
            .json(res.health),
        Err(e) => {
            let error = ErrorResponse {
                message: e.to_string()
            };
            HttpResponse::BadRequest()
                .append_header(("thread-id", index.to_string()))
                .json(error)
        }
    }
}
