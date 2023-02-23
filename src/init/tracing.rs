use anyhow::Result;
use chrono::Local;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    prelude::*,
    Registry,
};

use opentelemetry::global;

use crate::model::Config;

pub fn init_tracing(cfg: &Config) -> Result<()> {
    // time  format
    #[derive(Clone)]
    struct LocalTimer;

    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
        }
    }

    // file writer
    let info_file_appender = tracing_appender::rolling::daily("logs/info", "info.log")
        .with_min_level(Level::WARN)
        .with_max_level(Level::INFO);

    let access_file_appender = tracing_appender::rolling::daily("logs/access", "access.log")
        .with_max_level(Level::INFO)
        .with_filter(|meta| meta.target() == "access");

    let error_file_appender =
        tracing_appender::rolling::daily("logs/error", "error.log").with_max_level(Level::ERROR);

    let all_files = info_file_appender
        .and(error_file_appender)
        .and(access_file_appender);

    // common format
    let format = fmt::format()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_timer(LocalTimer);

    // opentelemetry
    let service_name = &cfg.service_name;
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // subscriber
    let subscriber = Registry::default()
        .with(opentelemetry)
        .with(
            fmt::Layer::new()
                .event_format(format.clone())
                .with_writer(std::io::stdout.with_max_level(Level::INFO)),
        )
        .with(
            fmt::Layer::new()
                .event_format(format)
                .with_ansi(false)
                .with_writer(all_files),
        );

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
