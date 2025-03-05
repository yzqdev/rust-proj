use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "asta", version = "1.0", author = "yzqdev")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Foo(FooCommand),
    Bar,
}

#[derive(Subcommand)]
enum FooCommand {
    #[command(subcommand)]
    Baz,
    #[command(subcommand)]
    Qux,
}

#[derive(Parser)]
struct BazCommand {
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Parser)]
struct QuxCommand {
    #[arg(short, long)]
    number: u32,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Foo(foo)) => match foo {
            FooCommand::Baz => {
                let baz_args = BazCommand::parse();
                println!("Baz command with verbose: {}", baz_args.verbose);
            }
            FooCommand::Qux => {
                let qux_args = QuxCommand::parse();
                println!("Qux command with number: {}", qux_args.number);
            }
        },
        Some(Commands::Bar) => println!("Bar command"),
        None => println!("No command specified"),
    }
}
