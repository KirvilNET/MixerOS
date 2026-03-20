pub mod engine;
pub mod dasp;
pub mod router;
pub mod system;
pub mod cli;
pub mod web;

use crate::engine::{ Engine };
use crate::system::StateManager;
use crate::cli::*;
use crate::web::server::WebServer;

  
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
  let rt  = tokio::runtime::Runtime::new().unwrap();

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
  tui.launch();
  
  let mut proc = Engine::new(config.name, config.channels, config.bus, config.sample_rate, config.buffer_size);
  
  proc.start().unwrap();
  proc.run().await;

  let webserver_thread = rt.spawn(async move {
    web.start().await;
  }).await;

}
