use clap::{Parser, Subcommand};

/// Verifies expressions against environment variables
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// force color mode (defaults to check tty)
    #[arg(long)]
    pub color: bool,

    /// force no-color mode (defaults to check tty)
    #[arg(long)]
    pub no_color: bool,

    /// prepend time to each log line
    #[arg(long)]
    pub log_time: bool,

    /// Turn general verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Configure component wise logging
    #[arg(long, short, action = clap::ArgAction::Append)]
    pub log: Option<Vec<String>>,

    #[command(subcommand)]
    pub action: Option<Actions>,
}

#[derive(Subcommand)]
pub enum Actions {
    Concat {

        /// Files containing timestamped data
        #[arg(required = true)]
        files: Option<Vec<String>>,

    },
}
