use crate::config::ServerConfig;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::Router;
use crate::app::AppState;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state:AppState, router:Router<AppState>) -> anyhow::Result<()> {
        let router = self.builder_router(state, router);
        let addr = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(addr).await?;

        tracing::info!("Listening on http://{}", listener.local_addr()?);

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn builder_router(&self, state:AppState, router:Router<AppState>) -> Router {
        Router::new()
            .merge(router)
            .with_state(state)
    }
}
