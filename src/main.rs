use crate::config::{initialize};
use crate::config::thread::{thread_counter, thread_index};
use actix_web::{App, HttpServer};
use crate::modules::shared::init_state;

mod config;
mod modules;

#[macro_use]
extern crate cherry_derive;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let address = initialize().await;
    let thread_counter = thread_counter();
    HttpServer::new(move || {
        App::new()
            .app_data(thread_index(thread_counter.clone()))
            .configure(modules::health::controllers::routes)
    })
        .bind(&address)
        .unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server: {:?}", err))
        .run()
        .await
}