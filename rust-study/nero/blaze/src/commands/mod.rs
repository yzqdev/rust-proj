pub mod clean;
pub mod dedup;
pub mod find;
pub mod hash;
pub mod info;
pub mod organize;
pub mod rename;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Find files by various criteria
    Find(find::FindArgs),

    /// Show file or directory information
    Info(info::InfoArgs),

    /// Compute file hashes (md5, sha1, sha256, sha512)
    Hash(hash::HashArgs),

    /// Batch rename files with patterns
    Rename(rename::RenameArgs),

    /// Organize files into directories by type/date
    Organize(organize::OrganizeArgs),

    /// Find duplicate files
    Dedup(dedup::DedupArgs),

    /// Clean up temporary and junk files
    Clean(clean::CleanArgs),
}
