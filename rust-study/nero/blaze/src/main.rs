mod cli;
mod commands;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Find(ref args) => commands::find::execute(args),
        commands::Commands::Info(ref args) => commands::info::execute(args),
        commands::Commands::Hash(ref args) => commands::hash::execute(args),
        commands::Commands::Rename(ref args) => commands::rename::execute(args),
        commands::Commands::Organize(ref args) => commands::organize::execute(args),
        commands::Commands::Dedup(ref args) => commands::dedup::execute(args),
        commands::Commands::Clean(ref args) => commands::clean::execute(args),
    }
}
