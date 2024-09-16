use anyhow::Result;
use chat::{get_router, AppConfig};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;

    let addr = format!("0.0.0.0:{}", config.server.post);
    info!("Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let app = get_router(config);
    axum::serve(listener, app).await?;

    Ok(())
}
