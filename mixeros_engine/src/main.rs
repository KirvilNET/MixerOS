pub mod engine;
pub mod dasp;
pub mod router;
pub mod system;
pub mod cli;
pub mod web;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::engine::{ Engine };  
use crate::system::StateManager;
use crate::cli::*;
use crate::web::server::Networking;
use ctrlc;

  
#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() {
  let rt  = tokio::runtime::Runtime::new().unwrap();

  let mut sm = StateManager::new().await;
  let mut config: system::state::EngineConfig = Default::default();

  match sm.init().await.ok() {
    Some(x) => config = x,
    None => {
      let _ = sm.create_store().await; 
    }
  }

  let tui = Tui::new(config.clone());
  tui.launch();
  
  let mut proc = Engine::new(config.name,  config.sample_rate, config.buffer_size, tui.table, None);
  
  proc.start(config.channels, config.buses).await.unwrap();
  proc.run().await;

  let mut web = Networking::new(config.webserver_port, Arc::new(RwLock::new(sm)), Arc::new(RwLock::new(proc)));
  let _web_handle = web.start_web().await.unwrap();
  let _rpc_handle = web.start_rpc().await.unwrap();

}
