

#[cfg(feature = "feature-log-panics")]
extern crate log;

#[cfg(feature = "feature-log-panics")]
use log::LevelFilter;
use std::fs::{OpenOptions};
use chrono::{DateTime, Utc};

#[cfg(feature = "feature-log-panics")]
pub fn log_init() {

    let file = OpenOptions::new()
        .append(true).create(true).read(true)
        .open("data/logs/rust_g.log");

    if let Ok(file) = file {
        simple_logging::log_to(
            file,
            LevelFilter::Info
        )
    } else {
        simple_logging::log_to_stderr(LevelFilter::Info)
    }

    log_panics::init();

    let now: DateTime<Utc> = Utc::now();

    log::info!("Rust G initialized, panic handler setup, PID: {}, Date: {}", std::process::id(), now);
}