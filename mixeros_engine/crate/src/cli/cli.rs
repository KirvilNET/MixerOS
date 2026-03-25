use crate::system::state::EngineConfig;
use whoami::*;
use yansi::Paint;
use std::io::{stdout, Write};
use crossterm::{ cursor, terminal::{self, ClearType}, ExecutableCommand };

use crate::cli::table::LiveTable;

pub struct Tui {
  config: EngineConfig,
  pub table: LiveTable,
}

const LOGO: &str = include_str!("../ascii/logo.txt");
const DIVIDER: &str = include_str!("../ascii/divider.txt");

const VERSION: &str = "1.0.0";

impl Tui {
  pub fn new(config: EngineConfig) -> Self {
    Self {
      config,
      table: LiveTable::new(vec!["ID", "Name", "Type", "Status"])
    }
  }

  pub fn launch(&self) {
    stdout().execute(cursor::Hide).ok();

    println!("{} v{}", LOGO.rgb(140, 82, 255), VERSION);
    println!("{}", DIVIDER);
    println!(" ");
    
  }

  pub fn print_system_info(&mut self) {
    println!("System Status");
    println!(" ");

    println!("{}@{}", username().unwrap_or("Unknown".to_string()).rgb(140, 82, 255), hostname().unwrap_or("localhost".to_string()).rgb(140, 82, 255));

    println!(" ");
    println!( "-------------" );
    println!(" ");

    self.table.draw();
  }

}

