use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., "database.host")
        key: String,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Plain)]
        format: OutputFormat,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Value to set
        value: String,

        /// Make the value secret (redacted in output)
        #[arg(short, long)]
        secret: bool,
    },

    /// List all configuration
    List {
        /// Filter by prefix
        #[arg(short, long)]
        prefix: Option<String>,

        /// Show secrets
        #[arg(long)]
        show_secrets: bool,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Plain)]
        format: OutputFormat,
    },

    /// Delete a configuration value
    Delete {
        /// Configuration key to delete
        key: String,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Plain,
    Json,
    Yaml,
}

pub fn execute(cmd: &ConfigCommands) {
    match cmd {
        ConfigCommands::Get { key, format } => {
            println!("Getting config '{key}' (format={format:?})");
        }
        ConfigCommands::Set { key, value, secret } => {
            println!("Setting config '{key}' to '{value}' (secret={secret})");
        }
        ConfigCommands::List { prefix, show_secrets, format } => {
            println!(
                "Listing configs (prefix={:?}, show_secrets={show_secrets}, format={format:?})",
                prefix.as_deref()
            );
        }
        ConfigCommands::Delete { key, force } => {
            println!("Deleting config '{key}' (force={force})");
        }
    }
}
