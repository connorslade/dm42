use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,

    /// Will print debug information after tokenizing
    #[clap(short, long)]
    pub debug: bool,

    /// Will print program statistics after transpiling
    #[clap(short, long)]
    pub stats: bool,
}
