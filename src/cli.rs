use clap::{ArgEnum, Parser, Subcommand};

pub fn init() -> Cli {
    Cli::parse()
}

#[derive(Debug, Parser)]
#[clap(name = "mococp")]
#[clap(about = "Moco CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Login {
        #[clap(arg_enum,default_value_t = Login::Moco)]
        system: Login,
    }, // login
    List {
        #[clap(long)]
        today: bool,

        #[clap(long)]
        week: bool,

        #[clap(long)]
        month: bool,
    }, // list existing moco entrys
    New {
        #[clap(long)]
        project: Option<i64>,

        #[clap(long)]
        task: Option<i64>,

        #[clap(long)]
        hours: Option<f64>,

        #[clap(long)]
        date: Option<String>,

        #[clap(long)]
        description: Option<String>,
    }, // create moco entry
    Add,  // add moco time exiting entry
    Edit, // edit moco time/description of exising entry
    Rm,   // delete moco entry
    Sync {
        #[clap(arg_enum,default_value_t = Sync::Jira)]
        system: Sync,

        #[clap(long)]
        today: bool,

        #[clap(long)]
        week: bool,

        #[clap(long)]
        month: bool,

        #[clap(long)]
        project: Option<i64>,

        #[clap(long)]
        task: Option<i64>,

        #[clap(long)]
        dry_run: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Login {
    Moco,
    Jira,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Sync {
    Jira,
}
