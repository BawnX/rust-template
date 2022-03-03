pub mod health_check;
use crate::modules::health::controllers::health_check::get_health_check;
use actix_web::web::{Data, get, ServiceConfig};
use crate::init_state;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.app_data(Data::new(init_state()));
    cfg.route("/health", get().to(get_health_check));
}
