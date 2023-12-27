use std::fs;

use anyhow::Result;
use image::{io::Reader, GenericImageView};

fn main() -> Result<()> {
    let image = Reader::open(r"img2.png")?.decode()?;
    assert_eq!(image.width(), 131);

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
                };

                byte |= (pixel as u8) << i;
            }
            buffer.push(byte);
        }
        lines.push(buffer);
        buffer = Vec::new();
    }

    println!("{} lines", lines.len());

    for (li, lines) in lines.chunks(10).enumerate() {
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

        fs::write(
            format!("out-{li}.free42"),
            format!("LBL \"PRNT{li}\"\n{prg}"),
        )?;
    }

    Ok(())
}

fn div_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

const CHARSET: &[char] = &[
    '÷', '×', '√', '∫', '\0', 'Σ', '\0', 'π', '¿', '≤', '␊', '≥', '≠', '\0', '↓', '→', '←', 'μ',
    '£', '°', 'Å', 'Ñ', 'Ä', '∡', 'ᴇ', 'Æ', '…', '\0', 'Ö', 'Ü', '\0', '•', ' ', '!', '\"', '#',
    '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\',
    ']', '↑', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⊦', '\0', '\0',
];

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

        if byte < CHARSET.len() as u8 {
            let chr = CHARSET[byte as usize];
            if chr != '\0' {
                buffer.push(chr);
                if chr == '\\' {
                    buffer.push('\\');
                }
                continue;
            }
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
