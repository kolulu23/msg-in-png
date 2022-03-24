use std::path::PathBuf;
use clap::{Parser, Subcommand, AppSettings};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    /// Optional name to operate on
    #[clap(subcommand)]
    pub command: Command,
    /// Path to target png file
    #[clap(short, parse(from_os_str), value_name = "FILE")]
    pub png: PathBuf,

}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add message into a png file
    Encode {
        chunk_type: String,
        message: String,
        #[clap(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    /// Get a message from a png file
    Decode {
        chunk_type: String,
    },
    /// Remove a message from a png file
    Remove {
        chunk_type: String,
    },
    /// Print given png file
    Print,
}
