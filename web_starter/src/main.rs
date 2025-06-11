mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;
mod api;
mod error;
mod response;
mod latency;
mod common;
mod serde;
mod query;
mod path;
mod json;
mod valid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = api::create_router();
    app::run(router).await?;
    Ok(())
}



