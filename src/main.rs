use crate::config::{initialize};
use crate::config::thread::{thread_counter, thread_index};
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::modules::shared::init_state;

mod config;
mod modules;


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let address = initialize().await;
    let thread_counter = thread_counter();
    let app_state = init_state().await;
    HttpServer::new(move || {
        App::new()
            .app_data(thread_index(thread_counter.clone()))
            .app_data(Data::new(app_state.clone()))
            .configure(modules::health::controllers::routes)
    })
        .bind(&address)
        .unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server: {:?}", err))
        .run()
        .await
}