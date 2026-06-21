use clap::Subcommand;

#[derive(Subcommand)]
pub enum DbCommands {
    /// Run database migrations
    Migrate {
        /// Migration version to apply (or "latest")
        #[arg(short, long, default_value = "latest")]
        version: String,

        /// Dry run without applying changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Seed the database with sample data
    Seed {
        /// Seed file path
        #[arg(short, long)]
        file: Option<String>,

        /// Number of records to generate
        #[arg(short, long, default_value_t = 100)]
        count: u32,
    },

    /// Reset database to initial state
    Reset {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

pub fn execute(cmd: &DbCommands) {
    match cmd {
        DbCommands::Migrate { version, dry_run } => {
            println!("Running migration to version '{version}' (dry_run={dry_run})");
        }
        DbCommands::Seed { file, count } => {
            println!("Seeding database: file={:?}, count={count}", file.as_deref());
        }
        DbCommands::Reset { yes } => {
            if *yes {
                println!("Database reset (confirmed)");
            } else {
                println!("Database reset requires --yes to confirm");
            }
        }
    }
}
