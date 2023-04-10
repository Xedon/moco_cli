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
    #[clap(about = "Login into (Moco/Jira)", long_about = None)]
    Login {
        #[clap(arg_enum, default_value_t = Login::Moco)]
        system: Login,
    },
    #[clap(about = "List activities", long_about = None)]
    List {
        #[clap(long)]
        today: bool,

        #[clap(long)]
        week: bool,

        #[clap(long)]
        last_week: bool,

        #[clap(long)]
        month: bool,

        #[clap(long)]
        last_month: bool,

        #[clap(long, help = "Sum up all activities of the day to one entry")]
        compact: bool,
    },
    #[clap(about = "Create new activity", long_about = None)]
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
    },
    #[clap(about = "Edit activity", long_about = None)]
    Edit {
        #[clap(long)]
        activity: Option<i64>,
    },
    #[clap(about = "Delete activity", long_about = None)]
    Rm {
        #[clap(long)]
        activity: Option<i64>,
    },
    #[clap(about = "Start/Stop activity timer", long_about = None)]
    Timer {
        #[clap(arg_enum)]
        system: Timer,

        #[clap(long)]
        activity: Option<i64>,
    },
    #[clap(about = "Sync missing Jira Tempo logs to Moco", long_about = None)]
    Sync {
        #[clap(arg_enum, default_value_t = Sync::Jira)]
        system: Sync,

        #[clap(long)]
        today: bool,

        #[clap(long)]
        week: bool,

        #[clap(long)]
        last_week: bool,

        #[clap(long)]
        month: bool,

        #[clap(long)]
        last_month: bool,

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
pub enum Timer {
    Start,
    Stop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Sync {
    Jira,
}
