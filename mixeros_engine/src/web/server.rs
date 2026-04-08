use axum::{
    routing::{ get },
    Router,
};
use std::{sync::Arc, thread::JoinHandle};
use tokio::sync::RwLock;

use crate::{engine::Engine, system::StateManager};
use crate::web::rpc::rpc_server::EngineRPC;

use super::http_api::{ metrics };


pub struct Networking {
    port: usize,
    state: Arc<RwLock<StateManager>>,
    engine: Arc<RwLock<Engine>>
}

impl Networking {
    
    pub fn new(port: usize, state: Arc<RwLock<StateManager>>, engine: Arc<RwLock<Engine>>) -> Self {
        Self {
            port,
            state,
            engine
        }
    }

    pub async fn start_rpc(&mut self) -> Result<(), std::io::Error> {
        let rpc = EngineRPC::new(self.state.clone(), self.engine.clone());
        rpc.start().await.unwrap();

        Ok(())
    }

    pub async fn start_web(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let port = self.port;
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .nest("/api/metrics", metrics::get_router());

        let thread = std::thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
                    axum::serve(listener, app).await.unwrap();
                });
        });

        Ok(thread)
    }
    
}