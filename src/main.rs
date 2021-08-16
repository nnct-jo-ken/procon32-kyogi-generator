mod fetch;
mod puzzle;

use puzzle::Pazzle;
use std::fs;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut puzzle = Pazzle::generate(None, None, None, None, None)?;

    puzzle.random_swap(50);
    puzzle.random_rotate(50);

    let mut writer = BufWriter::new(fs::File::create("image.ppm")?);

    puzzle.decode(&mut writer)?;

    Ok(())
}
