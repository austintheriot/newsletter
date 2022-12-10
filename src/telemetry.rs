use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing` subscriber
pub fn get_subscriber<Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static>(
    name: impl Into<String>,
    env_filter: impl AsRef<str>,
    sink: Sink,
) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

use std::sync::Once;

static START: Once = Once::new();

/// Only initializes subscriber on the first call--subsequent calls are ignored
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    START.call_once(|| {
        LogTracer::init().expect("Failed to set logger");
        set_global_default(subscriber).expect("Failed to set subscriber");
    });
}
