use axum::{
    routing::{ get },
    Router,
};

//use crate::system::state::EngineConfig;
use super::http_api::{ metrics };


pub struct WebServer {
    port: usize
}

impl WebServer {
    pub fn new(port: usize) -> Self {
        Self {
            port
        }
    }

    pub async fn start_rpc(&mut self) {

    }

    pub async fn start_web(&mut self) {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .nest("/api/metrics", metrics::get_router());

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }


    
}