use clap::{Parser, Subcommand};

/// A versatile CLI tool with multiple commands
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone with a friendly message
    Greet {
        /// Name of the person to greet
        name: String,
        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    /// Perform basic calculations
    Calc {
        /// First number
        a: f64,
        /// Operation: add, sub, mul, div
        #[arg(short, long)]
        op: String,
        /// Second number
        b: f64,
    },
    /// Echo the input
    Echo {
        /// Text to echo
        text: Vec<String>,
        /// Whether to echo in uppercase
        #[arg(short, long)]
        upper: bool,
    },
    /// Show current date/time
    Now {
        /// Format: date, time, datetime, timestamp
        #[arg(short, long, default_value_t = String::from("datetime"))]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, count } => {
            for _ in 0..count {
                println!("Hello, {}! Welcome to dante.", name);
            }
        }
        Commands::Calc { a, op, b } => {
            let result = match op.as_str() {
                "add" | "+" => a + b,
                "sub" | "-" => a - b,
                "mul" | "*" => a * b,
                "div" | "/" => {
                    if b == 0.0 {
                        eprintln!("Error: division by zero");
                        return;
                    }
                    a / b
                }
                _ => {
                    eprintln!("Unknown operation: {}. Use add/sub/mul/div", op);
                    return;
                }
            };
            println!("{} {} {} = {}", a, op, b, result);
        }
        Commands::Echo { text, upper } => {
            let msg = text.join(" ");
            if upper {
                println!("{}", msg.to_uppercase());
            } else {
                println!("{}", msg);
            }
        }
        Commands::Now { format } => {
            let now = chrono::Local::now();
            match format.as_str() {
                "date" => println!("{}", now.format("%Y-%m-%d")),
                "time" => println!("{}", now.format("%H:%M:%S")),
                "timestamp" => println!("{}", now.timestamp()),
                _ => println!("{}", now.format("%Y-%m-%d %H:%M:%S")),
            }
        }
    }
}
