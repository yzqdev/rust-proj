use clap::Subcommand;

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Generate a controller
    Controller {
        /// Controller name
        name: String,

        /// Generate with CRUD actions
        #[arg(short, long)]
        crud: bool,

        /// Actions to include
        #[arg(short, long, num_args = 1..)]
        actions: Vec<String>,
    },

    /// Generate a model
    Model {
        /// Model name
        name: String,

        /// Fields in format "name:type" (e.g., "age:i32", "email:String")
        #[arg(short, long, num_args = 1..)]
        fields: Vec<String>,

        /// Add timestamps (created_at, updated_at)
        #[arg(long)]
        timestamps: bool,
    },

    /// Generate a migration
    Migration {
        /// Migration description
        description: String,

        /// Database to target
        #[arg(long, default_value = "default")]
        database: String,
    },
}

pub fn execute(cmd: &GenerateCommands) {
    match cmd {
        GenerateCommands::Controller { name, crud, actions } => {
            println!("Generating controller '{name}' (crud={crud}, actions={actions:?})");
        }
        GenerateCommands::Model { name, fields, timestamps } => {
            println!(
                "Generating model '{name}' with fields {fields:?} (timestamps={timestamps})"
            );
        }
        GenerateCommands::Migration { description, database } => {
            println!("Creating migration '{description}' for database '{database}'");
        }
    }
}
