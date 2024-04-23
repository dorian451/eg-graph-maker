use leptos::{mount_to_body, view};
use std::{env, error::Error};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_subscriber_wasm::MakeConsoleWriter;

fn main() -> Result<(), Box<dyn Error>> {
    console_error_panic_hook::set_once();

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer().with_span_events(
            if let Ok("1") = env::var("RUST_LOG_TRACE_SPAN").as_deref() {
                FmtSpan::NEW | FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            },
        ))
        .with(Layer::new().with_writer(MakeConsoleWriter::default()))
        .try_init()?;

    mount_to_body(|| view! { <p>"Hello, world!"</p> });

    Ok(())
}
