#![warn(clippy::unwrap_used)]

use crate::{config::Config, notifier::Notifier};
use simple_logger::SimpleLogger;
use std::sync::Arc;

mod config;
mod monitor;
mod notifier;
mod subs;
mod tg_bot;

#[tokio::main]
async fn main() {
    let cfg = Arc::new(Config::read().expect("failed to read config"));

    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_level("sysmon", cfg.log_level)
        .init()
        .expect("failed to init logger");

    log::info!("starting service: config={:?}", cfg);

    let notifier = Arc::new(Notifier::init(cfg.clone()).expect("failed to init notifier"));

    let command_server_task = {
        let notifier = notifier.clone();
        async move {
            notifier.start_tg_command_server().await;
        }
    };

    let monitor_task = {
        let cfg = cfg.clone();
        let notifier = notifier.clone();
        async move {
            monitor::start(cfg, notifier)
                .await
                .expect("failed while running monitoring");
        }
    };

    tokio::join!(command_server_task, monitor_task);
}
