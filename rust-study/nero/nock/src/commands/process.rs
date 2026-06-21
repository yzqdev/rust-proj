use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct ProcessArgs {
    /// Input files
    #[arg(required = true, num_args = 1..)]
    pub files: Vec<String>,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output format
    #[arg(short, long, value_enum)]
    pub format: ProcessFormat,

    /// Enable parallel processing
    #[arg(short, long)]
    pub parallel: bool,

    /// Number of parallel workers (requires --parallel)
    #[arg(short, long, default_value_t = 4, requires = "parallel")]
    pub workers: u8,

    /// Compression level (0-9)
    #[arg(short = 'l', long, default_value_t = 6)]
    pub level: u8,

    /// Exclude patterns
    #[arg(long, num_args = 0..)]
    pub exclude: Vec<String>,

    /// Overwrite existing files
    #[arg(long)]
    pub overwrite: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ProcessFormat {
    Pdf,
    Html,
    Markdown,
    Plain,
}

pub fn execute(args: &ProcessArgs) {
    println!(
        "Processing {:?} -> {:?} (format={:?}, parallel={}, workers={}, level={}, exclude={:?}, overwrite={})",
        args.files,
        args.output.as_deref(),
        args.format,
        args.parallel,
        args.workers,
        args.level,
        args.exclude,
        args.overwrite
    );
}
