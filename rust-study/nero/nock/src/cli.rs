use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(
    name = "nock",
    version,
    about = "A multi-purpose CLI tool for learning clap",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
