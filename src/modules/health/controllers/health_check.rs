use crate::config::cqrs::query::Query;
use crate::modules::health::models::ObtainHealthQuery;
use actix_web::web::Data;
use actix_web::HttpResponse;
use tracing::instrument;

#[instrument]
pub async fn get_health_check(index: Data<u16>) -> HttpResponse {
    let health = ObtainHealthQuery::new(&mut ObtainHealthQuery::default(), true)
        .execute()
        .await;
    match health {
        Ok(health) => HttpResponse::Ok()
            .set_header("thread-id", index.to_string())
            .json(health),
        Err(_) => HttpResponse::NotFound().body("Not Found"),
    }
}
