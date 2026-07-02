use std::fs;
use std::io::Write;
use std::path::Path;

use clap::Parser;

/// Simulate installing a package
#[derive(Parser, Debug, Default)]
pub struct Install {
    /// Package name to install
    pub name: Option<String>,

    /// Install the latest version
    #[clap(long)]
    pub latest: bool,

    /// Additional parameter
    #[clap(long)]
    pub param: Option<String>,

    /// Install globally
    #[clap(short, long)]
    pub global: bool,
}

impl Install {
    pub(crate) fn call(&self) {
        let name = self
            .name
            .as_deref()
            .unwrap_or("(default package)");

        let target_dir = if self.global {
            "/usr/local/lib/phoebe"
        } else {
            "./node_modules"
        };

        if self.latest {
            println!("Installing latest version of '{}'", name);
        } else {
            println!("Installing '{}'", name);
        }

        if let Some(p) = &self.param {
            println!("  with parameter: {}", p);
        }

        // Create the target directory and a placeholder
        if !Path::new(target_dir).exists() {
            if let Err(e) = fs::create_dir_all(target_dir) {
                eprintln!("Warning: cannot create '{}': {}", target_dir, e);
            }
        }

        // Write a mock package.json
        let pkg_path = format!("{}/{}.json", target_dir, name);
        if let Ok(mut f) = fs::File::create(&pkg_path) {
            writeln!(
                f,
                r#"{{ "name": "{}", "version": "1.0.0", "installed": true }}"#,
                name
            )
            .ok();
            println!("Installed '{}' at {}", name, pkg_path);
        } else {
            println!("Installed '{}' (simulated)", name);
        }

        if let Some(p) = &self.param {
            println!("parameter is: {}", p);
        }
    }
}
