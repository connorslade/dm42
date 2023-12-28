use anyhow::Result;
use clap::Parser;

mod args;
mod data;
mod image;
mod misc;
mod text;
use args::{Args, SubCommand};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        SubCommand::Image(args) => image::encode(args)?,
        SubCommand::Text(args) => text::encode(args)?,
    }

    Ok(())
}
