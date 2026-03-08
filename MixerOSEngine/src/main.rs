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
async fn main() {
  let mut sm = StateManager::new().await;
  let mut config: system::state::EngineConfig = Default::default();

  match sm.init().await.ok() {
    Some(x) => config = x,
    None => {
      let _ = sm.create_store().await; 
    }
  }

  Tui::launch();

  if config != system::state::EngineConfig::default() {
    let engine_thread = tokio::spawn(async move {
      let host = engine::select_host().unwrap();

      let mut proc = engine::Engine::new(host, config.channels, config.bus, config.bit_depth, config.sample_rate, config.buffer_size);
      let _ = proc.start();

      loop {
        let _ = proc.run();
      }
    });
  }
}
