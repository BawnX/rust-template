use crate::config::initialize;
use crate::config::thread::{thread_counter, thread_index};
use actix_web::{App, HttpServer};

mod config;
mod modules;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = initialize();
    let thread_counter = thread_counter();
    HttpServer::new(move || {
        App::new()
            .data(thread_index(thread_counter.clone()))
            .configure(modules::health::controllers::routes)
    })
    .bind(&address)
    .unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server: {:?}", err))
    .run()
    .await
}
