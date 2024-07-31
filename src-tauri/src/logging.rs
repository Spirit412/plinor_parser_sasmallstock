#![allow(dead_code, unused_variables)]
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use tracing_subscriber::fmt::{self, SubscriberBuilder};
use std::fs::File;
use std::io::Write;

pub fn init_logging() {
    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));

    let file = File::create("app.log").expect("Unable to create log file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(file)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    set_global_default(subscriber).expect("Setting default subscriber failed");
}
