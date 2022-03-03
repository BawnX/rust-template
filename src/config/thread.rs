use crate::config::tracing::log;
use actix_web::web::Data;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

pub fn thread_counter() -> Arc<AtomicU16> {
    Arc::new(AtomicU16::new(1))
}

pub fn thread_index(thread_counter: Arc<AtomicU16>) -> Data<u16> {
    let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
    log::trace!("Starting Threads {}", thread_index);
    Data::new(thread_index)
}
