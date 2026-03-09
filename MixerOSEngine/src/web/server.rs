use tokio::net::*;
use socketioxide::{
    SocketIo, extract::{AckSender, Data, SocketRef}, handler::Value
};
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

        let (layer, io) = SocketIo::new_layer();

        io.ns("/", WebServer::on_connect);

        
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(layer);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
        println!("Client Connected: namespace: {} id: {}", socket.ns(), socket.id);
        
        socket.on("get-config", async |socket: SocketRef, Data::<Value>(data)| {
            println!("New message: {:?}", data);
        });
    }
    
}