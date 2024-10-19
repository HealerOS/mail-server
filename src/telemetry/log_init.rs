use std::io::stdout;
use tracing::subscriber::set_global_default;
use tracing::{info, Subscriber};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Sync + Send {
    let formatting_layer = fmt::layer().pretty();

    //todo 把日志输出到文件中
    //todo 解决没有requestId的问题

    Registry::default()
        .with(EnvFilter::new(env_filter))
        .with(BunyanFormattingLayer::new(name, stdout))
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Fail to setup log tracer.");
    set_global_default(subscriber).expect("Failed to set subscriber");
    info!("Log Tracer initialized");
}
