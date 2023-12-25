use anyhow::Result;
use image::{io::Reader, GenericImageView};

fn main() -> Result<()> {
    let image = Reader::open("image.bmp")?.decode()?;
    assert_eq!(image.dimensions(), (131, 16));

    Ok(())
}
