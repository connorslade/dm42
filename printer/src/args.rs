use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    /// Encode an image into a program for printing.
    Image(ImageArgs),
}

#[derive(Parser)]
pub struct ImageArgs {
    /// The input image.
    /// The width must be 131 pixels, but the height can be anything.
    pub input: PathBuf,
    /// The output program.
    /// If the path needs to be split into multiple files, they will be named like this: `{name}-{number}.{ext}`.
    pub output: PathBuf,
    /// The number of lines to print per program.
    /// If the total number of lines is less than this, only one program will be generated.
    #[clap(long, default_value_t = 10)]
    pub program_lines: usize,
}
