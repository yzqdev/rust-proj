use std::fs::File;
use std::io::{BufReader, Write};
use std::process::ExitCode;

use clap::CommandFactory;
use clap::Parser;
use clap_complete::Shell;
use phoebe::csv_to_json;

mod commands;

/// phoebe - CSV processor & package installer
#[derive(Debug, Parser)]
#[command(
    name = "phoebe",
    version,
    author,
    about = "CSV processor & package installer",
    long_about = None
)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    /// Convert CSV files to JSON
    #[command(name = "csv")]
    Csv(CsvOpts),
    /// Package installer simulation
    #[command(name = "install")]
    Install(commands::install::Install),
    /// Generate shell completions
    #[command(name = "completions")]
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input CSV file path
    #[arg(short, long, default_value = "input.csv")]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.json")]
    output: String,

    /// Field delimiter character
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// Whether the CSV has a header row
    #[arg(
        long,
        default_value_t = true,
        default_missing_value = "true",
        num_args = 0..=1,
        require_equals = true,
        action = clap::ArgAction::Set
    )]
    header: bool,

    /// Pretty-print the JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

impl CsvOpts {
    fn call(&self) -> Result<(), String> {
        let file =
            File::open(&self.input).map_err(|e| format!("cannot open '{}': {e}", self.input))?;
        let reader = BufReader::new(file);

        let conv = csv_to_json(reader, self.delimiter, self.header, self.pretty)
            .map_err(|e| e.to_string())?;

        for warning in &conv.warnings {
            eprintln!("Warning: {warning}");
        }

        let mut out_file = File::create(&self.output)
            .map_err(|e| format!("cannot write '{}': {e}", self.output))?;
        writeln!(out_file, "{}", conv.json)
            .map_err(|e| format!("cannot write '{}': {e}", self.output))?;

        println!(
            "Converted {} records from '{}' -> '{}'",
            conv.count, self.input, self.output
        );
        Ok(())
    }
}

fn main() -> ExitCode {
    let opts = Opts::parse();
    let result = match opts.cmd {
        SubCommand::Csv(csv) => csv.call(),
        SubCommand::Install(install) => {
            install.call();
            Ok(())
        }
        SubCommand::Completions { shell } => {
            let mut cmd = Opts::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(shell, &mut cmd, name, &mut out);
            print!("{}", String::from_utf8_lossy(&out));
            Ok(())
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("Error: {message}");
            ExitCode::FAILURE
        }
    }
}
