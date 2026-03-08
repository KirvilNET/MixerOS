use crate::system::state::EngineConfig;

pub struct Tui {
  config: EngineConfig
}

const LOGO: &str = include_str!("./ascii/logo.txt");
const DIVIDER: &str = include_str!("./ascii/divider.txt");
const VERSION: &str = "1.0.0";

impl Tui {
  pub fn launch() {
    println!("{} v{}", LOGO, VERSION);
    println!("{}", DIVIDER);
    
    println!("System Status");

  }
}

