use clap::{Parser, Subcommand, ValueEnum};

pub fn init() -> Cli {
    Cli::parse()
}

#[derive(Debug, Parser)]
#[clap(name = "mococp")]
#[clap(about = "Moco CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, help = "Show additional information for bug reports")]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Login into (Moco/Jira)", long_about = None)]
    Login {
        #[clap(value_enum, default_value_t = Login::Moco)]
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
        #[clap(long, help = "Optional project id for the activity")]
        project: Option<i64>,

        #[clap(long, help = "Optional task id for the activity")]
        task: Option<i64>,

        #[clap(long, help = "Optional hours in format (h.m)")]
        hours: Option<f64>,

        #[clap(long, help = "Optional date in format (YYYY-mm-dd)")]
        date: Option<String>,

        #[clap(long, help = "Optional description for the activity")]
        description: Option<String>,
    },
    #[clap(about = "Edit activity", long_about = None)]
    Edit {
        #[clap(long, help = "Optional activity id")]
        activity: Option<i64>,
    },
    #[clap(about = "Delete activity", long_about = None)]
    Rm {
        #[clap(long, help = "Optional activity id")]
        activity: Option<i64>,
    },
    #[clap(about = "Start/Stop activity timer", long_about = None)]
    Timer {
        #[clap(value_enum)]
        system: Timer,

        #[clap(long, help = "Optional activity id")]
        activity: Option<i64>,
    },
    #[clap(about = "Sync missing Jira Tempo logs to Moco", long_about = None)]
    Sync {
        #[clap(value_enum, default_value_t = Sync::Jira)]
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

        #[clap(long, help = "Optional project id for the activity")]
        project: Option<i64>,

        #[clap(long, help = "Optional task id for the activity")]
        task: Option<i64>,

        #[clap(long, help = "Just list what will be booked in moco from Jira")]
        dry_run: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Login {
    Moco,
    Jira,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Timer {
    Start,
    Stop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Sync {
    Jira,
}
