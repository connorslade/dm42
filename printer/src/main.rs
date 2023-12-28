use anyhow::Result;
use clap::Parser;

mod args;
mod data;
mod image;
mod misc;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        args::SubCommand::Image(image_args) => image::encode(image_args)?,
    }

    Ok(())
}
