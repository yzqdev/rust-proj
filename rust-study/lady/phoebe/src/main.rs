use clap::Parser;
mod commands;
mod cli;
#[derive(Debug, Parser)]
#[command(name="phoebe", version, author, about, long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show SCV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name="install",about="install")]
    Install(commands::install::Install),
    
}
impl  SubCommand {  
    pub fn call(self){
        match self {
            Self::Csv(cmd)=>cmd.call(),
            Self::Install(cmd)=>cmd.call(),
        }
    }
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,default_value="hello")]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
impl CsvOpts {
    fn call(&self) {
         println!("{}","hello world")
    }
}

fn main() {
    let opts = Opts::parse();
    opts.cmd.call();
  
}