mod installer;
mod system;

use std::env;
use clap::*;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);


#[derive(Parser)]
#[command(name = "mixeros")]
#[command(about = "Installs, uninstalls, or updates for MixerOS")]
#[command(bin_name = "mixeros")]
#[command(styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(ValueEnum, Clone, PartialEq)]
enum Features {
    #[value(name = "engine")]
    Engine,

    #[value(name = "ui")]
    Ui,

    #[value(name = "full")]
    Full
}

impl Into<String> for Features {
    fn into(self) -> String {
        match self {
            Features::Engine => "engine".to_string(),
            Features::Ui => "ui".to_string(),
            Features::Full => "full".to_string(),
        }
    }
}

#[derive(Subcommand, Clone)]
enum Commands {

    Install {
        #[arg(short, long)]
        source: Option<String>,

        #[arg(short, long)]
        dir: Option<String>,

        #[arg(short, long)]
        version: Option<String>,

        #[arg(short, long, default_value = "full")]
        feature: Option<Features>
    },

    Uninstall {
        #[arg(short, long)]
        version: String
    },

    Update,

    #[command(subcommand)]
    Dev
}

#[derive(Subcommand, Clone)]
enum Dev {
    Setup
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Install { source, dir, version, feature } => {
            installer::install(source, dir, version, feature).unwrap();
        },
        Commands::Uninstall { version } => {

        },
        Commands::Update => {

        },
        Commands::Dev => {

        },
    }
}   
