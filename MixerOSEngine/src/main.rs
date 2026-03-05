pub mod engine;
pub mod dasp;
pub mod router;
pub mod system;

use tokio::runtime::{ Runtime };

use crate::engine::{ Engine };
use crate::system::util::*;
use crate::system::StateManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  let rt = Runtime::new().unwrap();

  let sm = StateManager::new();


  let config: system::state::EngineConfig = sm.load_config().await.expect("could not load config");

  

  let dasp_engine = rt.spawn( async {
    let host = engine::select_host().unwrap();
    let engine = Engine::new(host, config.channels, config.bus, config.bit_depth, config.sample_rate);

    
  });

  //let dasp_interface = rt.spawn( async {});
}
