use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct DeployArgs {
    /// Target environment
    #[arg(short, long, value_enum)]
    pub env: DeployEnv,

    /// Dry run (preview without deploying)
    #[arg(long)]
    pub dry_run: bool,

    /// Git tag to deploy
    #[arg(short, long)]
    pub tag: Option<String>,

    /// Image tag (for container deployments)
    #[arg(long)]
    pub image: Option<String>,

    /// Number of replicas
    #[arg(short, long, default_value_t = 1)]
    pub replicas: u32,

    /// Skip tests before deploying
    #[arg(long)]
    pub skip_tests: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DeployEnv {
    Dev,
    Staging,
    Production,
}

pub fn execute(args: &DeployArgs) {
    println!(
        "Deploying to {:?} (dry_run={}, tag={:?}, image={:?}, replicas={}, skip_tests={})",
        args.env, args.dry_run, args.tag, args.image, args.replicas, args.skip_tests
    );
}
