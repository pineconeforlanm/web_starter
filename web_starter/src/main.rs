mod api;
mod app;
mod common;
mod config;
mod database;
mod entity;
mod error;
mod id;
mod json;
mod latency;
mod logger;
mod path;
mod query;
mod response;
mod serde;
mod server;
mod valid;
mod validation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = api::create_router();
    app::run(router).await?;
    Ok(())
}
