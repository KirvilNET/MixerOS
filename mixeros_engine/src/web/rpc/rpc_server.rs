use super::rpc_callbacks;
use crate::engine::Engine;
use crate::system::state::{DASPState, EngineConfig, StateManager};
use crate::system::util::EngineRole;
use crate::web::rpc;
use futures::{AsyncReadExt, TryFutureExt};
use mixeros_protocol::mixeros_protocol_sys::proto::{include::util_capnp, *};

use std::sync::{ Arc };
use std::thread::{ JoinHandle };

use tokio::net::*;
use tokio_util;
use tokio::sync::RwLock;

pub struct EngineRPC {
    state_manager: Arc<RwLock<StateManager>>,
    engine: Arc<RwLock<Engine>>,
    thread: Option<JoinHandle<()>>
}

impl EngineRPC {
    pub fn new(state_manager: Arc<RwLock<StateManager>>, engine: Arc<RwLock<Engine>>) -> Self {
        Self {
            state_manager,
            engine,
            thread: None
        }
    }

    pub async fn start(mut self) -> Result<(), Box<dyn std::error::Error>> {

        let state_manager = self.state_manager.clone();
        let engine = self.engine.clone();

        let thread = std::thread::spawn(move || {
            let connection_string = format!(
                "0.0.0.0:{}",
                self.state_manager.blocking_write().get_config().unwrap().rpc_port
            );
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let local = tokio::task::LocalSet::new();
            let clients: tokio::sync::RwLock<Vec<core::net::SocketAddr>> = tokio::sync::RwLock::new(Vec::new());

            local.block_on(&rt, async move {
                let engine_info = rpc_callbacks::engine::EngineImpl::new(
                    state_manager, engine
                );

                let engine_info_client: engine_capnp::engine::Client =
                    capnp_rpc::new_client(engine_info);

                let listener = TcpListener::bind(connection_string).await.unwrap();
                
                loop {
                    let (stream, addr) = listener.accept().await.unwrap();
                    clients.write().await.push(addr);

                    stream.set_nodelay(true).unwrap();

                    let (reader, writer) =
                        tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                    let network = capnp_rpc::twoparty::VatNetwork::new(
                        futures::io::BufReader::new(reader),
                        futures::io::BufWriter::new(writer),
                        capnp_rpc::rpc_twoparty_capnp::Side::Server,
                        Default::default(),
                    );

                    let rpc_system = capnp_rpc::RpcSystem::new(
                        Box::new(network),
                        Some(engine_info_client.clone().client),
                    );

                    tokio::task::spawn_local(
                        rpc_system.map_err(|e| println!("rpc error: {e:?}"))
                    );
                }
            });
        });
        self.thread = Some(thread);
        Ok(())
    }
}
