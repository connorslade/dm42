use std::fs;

use anyhow::Result;

use crate::args::TextArgs;

pub fn encode(args: TextArgs) -> Result<()> {
    let text = fs::read_to_string(args.input)?;
    let lines = text
        .lines()
        .map(|l| truncate(l, 22).trim_end())
        .collect::<Vec<_>>();

    if args.preview {
        for line in &lines {
            println!("|{:<22}|", line);
        }
    }

    if args.dry_run {
        return Ok(());
    }

    let mut prg = String::new();

    for line in lines {
        prg.push_str(&encode_string(line));
        prg.push_str("AVIEW\n");
    }

    fs::write(args.output, prg)?;
    Ok(())
}

fn truncate(s: &str, len: usize) -> &str {
    &s[..(len.min(s.len()))]
}

fn encode_string(s: &str) -> String {
    let mut prg = format!("\"{}\"\n", truncate(s, 14));

    if prg.len() <= 14 {
        return prg;
    }

    let mut i = 14;
    while i < s.len() {
        prg.push_str(&format!("├\"{}\"\n", truncate(&s[i..], 14)));
        i += 14;
    }

    prg
}
