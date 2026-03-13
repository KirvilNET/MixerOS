use tokio::net::*;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use crate::system::state::EngineConfig;


pub struct WebServer {
    port: usize,
}

impl WebServer {
    pub fn new(port: usize) -> Self {
        Self {
            port
        }
    }

    pub async fn start(&mut self) {

        
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    
}