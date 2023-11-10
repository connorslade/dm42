use std::fs;

use anyhow::Result;
use tokenize::Tokenizer;

mod codegen;
mod ident;
mod misc;
mod token;
mod tokenize;

const INPUT: &str = "test.dm42";
const OUTPUT: &str = "out.free42";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT)?;
    let tokens = Tokenizer::new(&input).tokenize()?;
    println!("{:#?}", tokens);
    let output = codegen::generate(tokens);
    fs::write(OUTPUT, output)?;

    Ok(())
}
