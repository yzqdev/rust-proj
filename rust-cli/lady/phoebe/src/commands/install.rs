#[derive(clap::Parser, Debug, Default)]
pub  struct Install{
    #[clap(long )]
    pub latest: bool,
    #[clap(long)]
    pub param:String,
}

impl Install {
    pub(crate) fn call(&self) {
        if self.latest {
            println!("latest")
        }
       if self.param.len()>0 {
           print!("{}",self.param)
       }
    }
}