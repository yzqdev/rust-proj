use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(
    name = "blaze",
    version,
    about = "A fast file processing CLI tool",
    long_about = None,
    after_help = "EXAMPLES:\n  blaze find . --ext rs --min-size 1kb\n  blaze hash ./src --algo sha256\n  blaze rename ./photos --pattern \"photo_{n}\" --ext jpg\n  blaze organize ./downloads --by extension\n  blaze dedup ./music --dry-run\n  blaze clean ./temp --min-age 7d"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
