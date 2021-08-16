mod fetch;
mod puzzle;

use puzzle::Pazzle;
use std::fs;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut puzzle = Pazzle::generate(None, None, None, None, None)?;

    puzzle.swap_tile((0, 0), (1, 1));
    puzzle.rotate_tile((0, 0), puzzle::RotateDirection::Rotate90);

    let mut writer = BufWriter::new(fs::File::create("image.ppm")?);

    puzzle.decode(&mut writer)?;

    Ok(())
}
