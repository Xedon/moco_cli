use clap::{Parser, Subcommand};

pub fn init() -> Cli {
    Cli::parse()
}

#[derive(Debug, Parser)]
#[clap(name = "mococp")]
#[clap(about = "Moco CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Login, // login
    List,  // list existing moco entrys
    New,   // create moco entry
    Add,   // add moco time exiting entry
    Edit,  // edit moco time/description of exising entry
    Rm,    // delete moco entry
}
