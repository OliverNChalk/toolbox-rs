use std::path::Path;

use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

#[must_use]
pub fn setup_tracing(pkg_name: &str, log_directory: Option<&Path>) -> Option<WorkerGuard> {
    // Setup stdout layer.
    let stdout_filter = EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .unwrap();
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(stdout_filter);

    // Setup file layer (if requested).
    let (file_layer, file_guard) = log_directory
        .map(|directory| {
            let file_appender =
                tracing_appender::rolling::hourly(directory, format!("{pkg_name}.log"));
            let (file_writer, file_guard) = tracing_appender::non_blocking(file_appender);

            // Load the user's file filter else fallback to the default.
            let file_filter = std::env::var("RUST_FILE_LOG").ok();
            let file_filter = file_filter.unwrap_or_else(|| format!("info,{pkg_name}=debug"));
            let file_filter: EnvFilter = file_filter.parse().unwrap();

            let file_layer = tracing_subscriber::fmt::layer()
                .json()
                .with_writer(file_writer)
                .with_filter(file_filter);

            (Some(file_layer), Some(file_guard))
        })
        .unwrap_or((None, None));

    // Combine stdout & file layer.
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    file_guard
}
