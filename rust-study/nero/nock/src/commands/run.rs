use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct RunArgs {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080, env = "NOCK_PORT")]
    pub port: u16,

    /// Host to bind to
    #[arg(short = 'H', long, default_value = "127.0.0.1", env = "NOCK_HOST")]
    pub host: String,

    /// Environment (reads from NOCK_ENV if not specified)
    #[arg(short, long, env = "NOCK_ENV", default_value = "development")]
    pub env: String,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Log format
    #[arg(long, value_enum, default_value_t = LogFormat::Pretty)]
    pub log_format: LogFormat,

    /// Config files to load
    #[arg(long, num_args = 0..)]
    pub config: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LogFormat {
    Pretty,
    Compact,
    Json,
}

pub fn execute(args: &RunArgs) {
    println!(
        "Starting server on {}:{} (env={}, verbose={}, log_format={:?}, configs={:?})",
        args.host, args.port, args.env, args.verbose, args.log_format, args.config
    );
}
