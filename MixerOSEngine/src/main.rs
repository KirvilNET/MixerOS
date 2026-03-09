pub mod engine;
pub mod dasp;
pub mod router;
pub mod system;
pub mod cli;
pub mod web;

use tokio::join;

use crate::engine::{ Engine };
use crate::system::StateManager;
use crate::cli::*;
use crate::web::server::WebServer;

#[tokio::main]
async fn main() {
  let mut sm = StateManager::new().await;
  let mut config: system::state::EngineConfig = Default::default();
  let mut web = WebServer::new(config.ws_port);

  match sm.init().await.ok() {
    Some(x) => config = x,
    None => {
      let _ = sm.create_store().await; 
    }
  }

  let tui = Tui::new(config.clone());

  tui.launch().await;

  if config != system::state::EngineConfig::default() {
    let engine_thread = tokio::spawn(async move {
      let host = engine::select_host().unwrap();

      let mut proc = Engine::new(host, config.channels, config.bus, config.bit_depth, config.sample_rate, config.buffer_size);

      proc.start().unwrap();
      proc.run().await;

    });

    
  }
  
  web.start().await;
  
}
