pub mod health_check;
use crate::modules::health::controllers::health_check::get_health_check;
use crate::modules::health::models::Health;
use actix_web::web::{get, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(get_health_check));
}
