use image::io::Reader;
use image::RgbImage;
use std::io::Cursor;

pub fn fetch_image(size: (u32, u32)) -> Result<RgbImage, Box<dyn std::error::Error>> {
    let data = fetch_data(size)?;
    convert_image(data)
}

fn fetch_data((x, y): (u32, u32)) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!("https://source.unsplash.com/random/{}x{}", x, y);

    let data = reqwest::blocking::get(url)?.bytes()?.to_vec();

    Ok(data)
}

fn convert_image(data: Vec<u8>) -> Result<RgbImage, Box<dyn std::error::Error>> {
    Ok(Reader::new(Cursor::new(data))
        .with_guessed_format()?
        .decode()?
        .to_rgb8())
}
