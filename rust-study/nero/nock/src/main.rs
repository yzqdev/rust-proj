mod cli;
mod commands;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Hello(ref args) => commands::hello::execute(args),
        commands::Commands::Init(ref args) => commands::init::execute(args),
        commands::Commands::Db(ref cmd) => commands::db::execute(cmd),
        commands::Commands::Generate(ref cmd) => commands::generate::execute(cmd),
        commands::Commands::Run(ref args) => commands::run::execute(args),
        commands::Commands::Deploy(ref args) => commands::deploy::execute(args),
        commands::Commands::Config(ref cmd) => commands::config::execute(cmd),
        commands::Commands::Process(ref args) => commands::process::execute(args),
    }
}
