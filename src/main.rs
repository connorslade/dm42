use std::fs;

use anyhow::Result;
use tokenize::Tokenizer;

mod ident;
mod tokenize;

const INPUT: &str = "test.dm42";
const OUTPUT: &str = "out.free42";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT)?;
    let tokens = Tokenizer::new(&input);
    let tokens = tokens.tokenize()?;
    println!("{:#?}", tokens);

    Ok(())
}
