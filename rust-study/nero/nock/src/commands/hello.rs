use clap::Args;

#[derive(Args)]
pub struct HelloArgs {
    /// Name of the person to greet
    #[arg(short, long, default_value = "World")]
    pub name: String,

    /// Number of times to say hello
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

pub fn execute(args: &HelloArgs) {
    for i in 0..args.count {
        println!("[{i}] Hello, {}!", args.name);
    }
}
