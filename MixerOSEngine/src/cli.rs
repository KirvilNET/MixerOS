use crate::system::state::EngineConfig;
use neofetch::*;
use serde::de::value;
use yansi::Paint;
pub struct Tui {
  config: EngineConfig
}

const LOGO: &str = include_str!("./ascii/logo.txt");
const DIVIDER: &str = include_str!("./ascii/divider.txt");

const VERSION: &str = "1.0.0";

impl Tui {
  pub fn new(config: EngineConfig) -> Self {
    Self {
      config
    }
  }

  pub async fn launch(&self) {
    println!("{} v{}", LOGO.rgb(140, 82, 255), VERSION);
    println!("{}", DIVIDER);
    println!(" ");
    
    self.print_system_info().await;
  }

  pub async fn print_system_info(&self) {
    println!("System Status");
    println!(" ");

    let fetch = Neofetch::new().await;

    if let Some((user, hname)) = fetch.user.ok().zip(fetch.hostname.ok()) {

      println!("{}@{}", user.rgb(140, 82, 255), hname.rgb(140, 82, 255));
    } 

    println!(" ");
    println!( "-------------" );
    println!(" ");

    if let Some(val) = fetch.cpu.ok() {
      println!("CPU: {}", val.to_string());
    } 

    if let Some(val) = fetch.memory.ok() {
      println!("Memory: {}", val.to_string());
    }

    if let Some(val) = fetch.gpu {
      for (id, gpu) in val.iter().enumerate() {
        println!("GPU{}: {}", id, gpu);
      } 
    }

    if let Some(val) = fetch.network.ok() {
      for net in val.iter() {
        println!("Network: {}", net)
      }
    }
    
    println!(" ");
  }
}

