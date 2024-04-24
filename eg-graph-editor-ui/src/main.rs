mod app;

use crate::app::App;
use leptos::{mount_to_body, view};
use std::{env, error::Error};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_panic::panic_hook;
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
        .with(Layer::new().without_time())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("EGE_LOG")
                .from_env()?,
        )
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_span_events(if let Ok("1") = env::var("EGE_LOG_TRACE_SPAN").as_deref() {
                    FmtSpan::NEW | FmtSpan::CLOSE
                } else {
                    FmtSpan::NONE
                }),
        )
        .with(
            Layer::new()
                .without_time()
                .with_writer(MakeConsoleWriter::default()),
        )
        .try_init()?;

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        prev_hook(panic_info);
        panic_hook(panic_info);
    }));

    mount_to_body(|| {
        view! { <App/> }
    });

    Ok(())
}
