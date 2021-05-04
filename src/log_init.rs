

#[cfg(feature = "feature-log-panics")]
extern crate log;

#[cfg(feature = "feature-log-panics")]
use log::LevelFilter;
use std::fs::{OpenOptions};
use chrono::{DateTime, Utc};

#[cfg(feature = "feature-log-panics")]
pub fn log_init() {
    simple_logging::log_to(
        OpenOptions::new()
            .append(true).create(true).read(true)
            .open("data/logs/rust_g.log").unwrap(),
        LevelFilter::Info
    );

    log_panics::init();

    let now: DateTime<Utc> = Utc::now();

    log::info!("Rust G initialized, panic handler setup, PID: {}, Date: {}", std::process::id(), now);
}