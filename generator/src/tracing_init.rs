use rootcause_tracing::RootcauseLayer;
use tracing_subscriber::{
    Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

pub fn init_subscriber() {
    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("client", tracing::Level::DEBUG);

    let fmt_layer = tracing_subscriber::fmt::layer()
        //.pretty()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);

    let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

    tracing_subscriber::Registry::default()
        .with(RootcauseLayer)
        .with(fmt_layer_filtered)
        .init();
}
