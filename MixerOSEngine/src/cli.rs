use crate::system::state::EngineConfig;
use hardware_query::{ HardwareInfo };
use whoami::*;
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

  pub fn launch(&self) {
    println!("{} v{}", LOGO.rgb(140, 82, 255), VERSION);
    println!("{}", DIVIDER);
    println!(" ");

    match HardwareInfo::query() {
        Ok(info) => self.print_system_info(info),
        Err(err) => println!("Could not get hardware info Error: {}", err),
    }
    
    
  }

  pub fn print_system_info(&self, hw_info: HardwareInfo) {
    println!("System Status");
    println!(" ");



    println!("{}@{}", username().unwrap_or("Unknown".to_string()).rgb(140, 82, 255), hostname().unwrap_or("localhost".to_string()).rgb(140, 82, 255));

    println!(" ");
    println!( "-------------" );
    println!(" ");

    println!("CPU: {} ({} Core) {}hz", hw_info.cpu().model_name, hw_info.cpu().physical_cores, hw_info.cpu().max_frequency);
    println!("Memory: {} Gb", hw_info.memory().total_gb());

    for (id, gpu) in hw_info.gpus().iter().enumerate() {
      println!("GPU{}: {} {} {} Gb {:?}", id, gpu.vendor(), gpu.model_name(), gpu.memory_gb(), gpu.compute_capabilities);
    } 

    for net in hw_info.network_interfaces.iter() {
      println!("Network: {:?}", net)
    }
    
    println!(" ");
  }
}

