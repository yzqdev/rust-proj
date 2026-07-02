use clap::Parser;

mod cli;
mod commands;

#[derive(Debug, Parser)]
#[command(name = "phoebe", version, author, about = "CSV processor & package installer", long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Convert CSV files to JSON or other formats
    #[command(name = "csv")]
    Csv(CsvOpts),
    /// Package installer simulation
    #[command(name = "install")]
    Install(commands::install::Install),
}

impl SubCommand {
    pub fn call(self) {
        match self {
            Self::Csv(cmd) => cmd.call(),
            Self::Install(cmd) => cmd.call(),
        }
    }
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
    #[arg(long, default_value_t = true)]
    header: bool,

    /// Pretty-print the JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

impl CsvOpts {
    fn call(&self) {
        use std::fs::File;
        use std::io::BufReader;

        let file = match File::open(&self.input) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: cannot open '{}': {}", self.input, e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(self.delimiter as u8)
            .has_headers(self.header)
            .from_reader(reader);

        let headers: Vec<String> = if self.header {
            csv_reader
                .headers()
                .unwrap()
                .iter()
                .map(|h| h.to_string())
                .collect()
        } else {
            // Generate column names if no header
            let mut cols = Vec::new();
            if let Some(Ok(row)) = csv_reader.records().next() {
                for i in 0..row.len() {
                    cols.push(format!("column_{}", i));
                }
            }
            // Re-create reader since we already consumed one row
            drop(csv_reader);
            let file = File::open(&self.input).unwrap();
            let reader = BufReader::new(file);
            csv_reader = csv::ReaderBuilder::new()
                .delimiter(self.delimiter as u8)
                .has_headers(false)
                .from_reader(reader);
            cols
        };

        let mut records: Vec<serde_json::Value> = Vec::new();
        for result in csv_reader.records() {
            match result {
                Ok(record) => {
                    let mut obj = serde_json::Map::new();
                    for (i, field) in record.iter().enumerate() {
                        let key = headers.get(i).map(|s| s.as_str()).unwrap_or("col");
                        let key_str = if key == "col" {
                            format!("col_{}", i)
                        } else {
                            key.to_string()
                        };
                        obj.insert(key_str, serde_json::Value::String(field.to_string()));
                    }
                    records.push(serde_json::Value::Object(obj));
                }
                Err(e) => {
                    eprintln!("Warning: skipping row - {}", e);
                }
            }
        }

        let json = if self.pretty {
            serde_json::to_string_pretty(&records).unwrap()
        } else {
            serde_json::to_string(&records).unwrap()
        };

        let mut out_file = File::create(&self.output).unwrap_or_else(|e| {
            eprintln!("Error: cannot write '{}': {}", self.output, e);
            std::process::exit(1);
        });
        use std::io::Write;
        writeln!(out_file, "{}", json).unwrap();
        println!(
            "Converted {} records from '{}' -> '{}'",
            records.len(),
            self.input,
            self.output
        );
    }
}

fn main() {
    let opts = Opts::parse();
    opts.cmd.call();
}
