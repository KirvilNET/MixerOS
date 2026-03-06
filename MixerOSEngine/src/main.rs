pub mod engine;
pub mod dasp;
pub mod router;
pub mod system;
pub mod cli;
pub mod web;

use tokio::runtime::{ Runtime };

use crate::engine::{ Engine };
use crate::system::util::*;
use crate::system::StateManager;
use crate::cli::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  let rt = Runtime::new().unwrap();
  let sm = StateManager::new();
  let config: system::state::EngineConfig = sm.load_config().await.expect("could not load config");

  Tui::launch();

  let engine_thread = rt.spawn(async {
    let host = engine::select_host()

    if host.is_err() {
      panic!("Could not find a compatible host")
    }

    let proc = engine::Engine::new(host, ch, buses, bit_depth, sample_rate)
  });
  
  

  
}
