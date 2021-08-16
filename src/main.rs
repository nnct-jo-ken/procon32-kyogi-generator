mod fetch;
mod puzzle;

use puzzle::Pazzle;
use std::fs;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle = Pazzle::generate(None, None, None, None, None)?;

    let mut writer = BufWriter::new(fs::File::create("image.ppm")?);

    puzzle.decode(&mut writer)?;

    Ok(())
}
