pub mod config;
pub mod db;
pub mod deploy;
pub mod generate;
pub mod hello;
pub mod init;
pub mod process;
pub mod run;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Say hello to someone
    Hello(hello::HelloArgs),

    /// Initialize a new project
    Init(init::InitArgs),

    /// Manage database operations
    #[command(subcommand)]
    Db(db::DbCommands),

    /// Generate code scaffolding
    #[command(subcommand)]
    Generate(generate::GenerateCommands),

    /// Run the application server
    Run(run::RunArgs),

    /// Deploy to an environment
    Deploy(deploy::DeployArgs),

    /// Configuration management
    #[command(subcommand)]
    Config(config::ConfigCommands),

    /// Process and transform files
    Process(process::ProcessArgs),
}
