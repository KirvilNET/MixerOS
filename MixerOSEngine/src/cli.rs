pub struct CLI;

const LOGO: &str = include_str!("./ascii/logo.txt");
const VERSION: &str = "1.0.0";

impl CLI {
  pub fn launch() {
    println!("{}", LOGO);
    println!("-------------------------------------------------------");
    println!("v{}", VERSION);

    
  }
}