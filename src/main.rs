use std::fs;

use anyhow::Result;
use clap::Parser;
use codegen::CodeGen;
use tokenize::Tokenizer;

use crate::args::Args;

mod args;
mod codegen;
mod ident;
mod misc;
mod token;
mod tokenize;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("[*] Loading input");
    let input = fs::read_to_string(args.input)?;
    println!("[*] Tokenizing");
    let tokens = Tokenizer::new(&input).tokenize()?;

    if args.debug {
        println!("{:#?}", tokens);
    }

    println!("[*] Transpiling");
    let mut codegen = CodeGen::new();
    let output = codegen::generate(&mut codegen, tokens);
    println!("[*] Writing output");
    fs::write(args.output, output)?;

    if args.stats {
        println!("[*] Statistics");
        println!("   Functions: {}", codegen.used_functions());
        println!("Instructions: {}", codegen.used_instructions());
        println!(
            " Identifiers: {}/115 ({}%)",
            codegen.used_identifiers(),
            (codegen.used_identifiers() as f32 / 115.0 * 100.0).round()
        );
    }

    Ok(())
}
