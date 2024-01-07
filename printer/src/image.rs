use std::fs;

use anyhow::Result;
use image::{io::Reader, GenericImageView};

use crate::{
    args::ImageArgs,
    data::{CHARSET, INVALID_CHARS},
    misc::div_up,
};

pub fn encode(args: ImageArgs) -> Result<()> {
    let image = Reader::open(args.input)?.decode()?;
    assert_eq!(image.width(), 131);

    println!("[*] Image loaded ({}x{})", image.width(), image.height());

    let mut lines = Vec::new();
    let mut buffer = Vec::new();

    for ri in 0..div_up(image.height(), 8) {
        for ci in 0..image.width() {
            let mut byte = 0u8;
            for i in 0..8 {
                let pixel = if image.in_bounds(ci, ri * 8 + i) {
                    image.get_pixel(ci, ri * 8 + i)[0] == 0
                } else {
                    false
                } ^ args.invert;

                byte |= (pixel as u8) << i;
            }
            buffer.push(byte);
        }
        lines.push(buffer);
        buffer = Vec::new();
    }

    println!("[*] Image split into {} lines", lines.len());

    let out = args.output.file_stem().unwrap().to_str().unwrap();
    let folder = args.output.parent().unwrap();

    for (li, lines) in lines.chunks(args.program_lines).enumerate() {
        let mut prg = String::new();
        for (y, line) in lines.iter().enumerate() {
            if y % 2 == 0 {
                prg.push_str("CLLCD\n");
            }

            for (x, bytes) in line.chunks(44).enumerate() {
                let rng = encode_prg(bytes)?;
                prg.push_str(&rng);
                prg.push_str(&format!(
                    "{}\n{}\nAGRAPH\nDROPN 2\n",
                    y % 2 * 8 + 1,
                    x * 44 + 1
                ));
            }

            if y % 2 == 1 || y + 1 == lines.len() {
                prg.push_str("PRLCD\n");
                prg.push_str("STOP\n");
            }
        }

        if lines.len() <= args.program_lines {
            fs::write(&args.output, prg)?;
            break;
        }

        let path = folder.join(format!("{}-{:02}.prg", out, li));
        fs::write(&path, prg)?;
    }

    Ok(())
}

fn encode_prg(cols: &[u8]) -> Result<String> {
    debug_assert!(cols.len() <= 44);
    let mut prg = String::new();

    let mut buffer = String::new();
    prg.push_str("CLA\n");

    let flush = |buffer: &mut String, prg: &mut String| {
        if !buffer.is_empty() {
            prg.push_str(&format!("├\"{}\"\n", buffer));
            buffer.clear();
        }
    };

    let mut drop = 0;
    for &byte in cols {
        if buffer.chars().count() > 13 {
            flush(&mut buffer, &mut prg);
        }

        if byte < CHARSET.len() as u8 && !INVALID_CHARS.contains(&(byte as usize)) {
            let chr = CHARSET[byte as usize];
            buffer.push(chr);
            if chr == '\\' {
                buffer.push('\\');
            }
            continue;
        }

        flush(&mut buffer, &mut prg);
        prg.push_str(&format!("{}\nXTOA\n", byte));
        drop += 1;
    }

    flush(&mut buffer, &mut prg);

    while drop > 0 {
        prg.push_str(&format!("DROPN {}\n", drop.min(9)));
        drop -= 9;
    }

    Ok(prg)
}
