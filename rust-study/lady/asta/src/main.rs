use std::process::ExitCode;

use asta::{Cli, run};

fn main() -> ExitCode {
    let cli = Cli::parse_args();
    match run(&cli) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
