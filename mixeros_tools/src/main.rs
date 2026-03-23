use std::env;
use clap::*;

#[derive(Parser)]
#[command(
    name = "mixeros-setup",
    about = "MixerOS OpenCL + CUDA environment setup",
    version,
    propagate_version = true,
)]
struct Cmds {
    #[command(subcommand)]
    command: Tools
}


#[derive(Subcommand)]
enum Tools {
    Update
}

fn main() {
    let cli = Cmds::parse();

}   
