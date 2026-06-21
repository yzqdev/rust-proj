use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct InitArgs {
    /// Project name
    pub name: String,

    /// Template to use
    #[arg(short, long, value_enum, default_value_t = Template::Default)]
    pub template: Template,

    /// Target directory
    #[arg(short, long)]
    pub dir: Option<String>,

    /// Force overwrite existing files
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Template {
    Default,
    Web,
    Cli,
    Library,
}

pub fn execute(args: &InitArgs) {
    println!(
        "Initializing project '{}' with template {:?} (dir={:?}, force={})",
        args.name, args.template, args.dir, args.force
    );
}
