use crate::fetch::fetch_image;
use image::{imageops, RgbImage};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::io::{self, Write};

const MIN_IMAGE_SIZE: u32 = 32;
const MAX_IMAGE_SIZE: u32 = 2048;
const MIN_DIVISION_SIZE: u32 = 16;
const MAX_DIVISION_SIZE: u32 = 256;
const MIN_DIVISION: u32 = 2;
const MAX_DIVISION: u32 = 16;
const MIN_SELECT_LIMIT: u32 = 2;
const MAX_SELECT_LIMIT: u32 = 128;
const MIN_COST_RATE: u32 = 1;
const MAX_COST_RATE: u32 = 500;

#[derive(Debug)]
pub struct Puzzle {
    division: (u32, u32),
    division_size: u32,
    select_limit: u32,
    select_rate: u32,
    swap_rate: u32,
    image: RgbImage,
}

impl Puzzle {
    fn new(
        division_size: u32,
        select_limit: u32,
        select_rate: u32,
        swap_rate: u32,
        image: RgbImage,
    ) -> Self {
        Puzzle {
            division: (
                (image.width() / division_size),
                (image.height() / division_size),
            ),
            division_size,
            select_limit,
            select_rate,
            swap_rate,
            image,
        }
    }

    pub fn generate(
        division_size: Option<u32>,
        division: Option<(u32, u32)>,
        select_limit: Option<u32>,
        select_rate: Option<u32>,
        swap_rate: Option<u32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let division_size = match division_size {
            Some(x) => x,
            None => generate_division_size(),
        };
        let division = match division {
            Some(x) => x,
            None => generate_division(division_size),
        };
        let select_limit = match select_limit {
            Some(x) => x,
            None => generate_select_limit(),
        };
        let select_rate = match select_rate {
            Some(x) => x,
            None => generate_cost_rate(),
        };
        let swap_rate = match swap_rate {
            Some(x) => x,
            None => generate_cost_rate(),
        };

        let image = fetch_image((division.0 * division_size, division.1 * division_size))?;

        Ok(Self::new(
            division_size,
            select_limit,
            select_rate,
            swap_rate,
            image,
        ))
    }

    pub fn decode(&self, writable: &mut impl Write) -> io::Result<()> {
        writeln!(writable, "P6")?;
        writeln!(writable, "# {} {}", self.division.0, self.division.1)?;
        writeln!(writable, "# {}", self.select_limit)?;
        writeln!(writable, "# {} {}", self.select_rate, self.swap_rate)?;
        writeln!(writable, "{} {}", self.image.width(), self.image.height())?;
        writeln!(writable, "{}", 255)?;

        for pixel in self.image.pixels() {
            writable.write_all(pixel.0.as_ref())?;
        }

        Ok(())
    }

    fn swap_tile(&mut self, tile_a: (u32, u32), tile_b: (u32, u32)) {
        for x in 0..self.division_size {
            for y in 0..self.division_size {
                let p1 = self.image.get_pixel_mut(
                    tile_a.0 * self.division_size + x,
                    tile_a.1 * self.division_size + y,
                ) as *mut _;
                let p2 = self.image.get_pixel_mut(
                    tile_b.0 * self.division_size + x,
                    tile_b.1 * self.division_size + y,
                ) as *mut _;
                unsafe {
                    std::ptr::swap(p1, p2);
                }
            }
        }
    }

    fn rotate_tile(&mut self, (tile_x, tile_y): (u32, u32), direction: RotateDirection) {
        let mut tile = imageops::crop(
            &mut self.image,
            tile_x * self.division_size,
            tile_y * self.division_size,
            self.division_size,
            self.division_size,
        );

        let new_tile = match direction {
            RotateDirection::Rotate90 => imageops::rotate90(&mut tile),
            RotateDirection::Rotate180 => imageops::rotate180(&mut tile),
            RotateDirection::Rotate270 => imageops::rotate270(&mut tile),
        };

        for x in 0..self.division_size {
            for y in 0..self.division_size {
                let pixel = new_tile.get_pixel(x, y);

                self.image.put_pixel(
                    tile_x * self.division_size + x,
                    tile_y * self.division_size + y,
                    *pixel,
                );
            }
        }
    }

    fn select_random_tile(&self) -> (u32, u32) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..self.division.0);
        let y = rng.gen_range(0..self.division.1);
        (x, y)
    }

    fn select_random_tile_without_upper_left(&self) -> (u32, u32) {
        let mut tile = (0, 0);

        while tile.0 == 0 && tile.1 == 0 {
            tile = self.select_random_tile()
        }

        tile
    }

    pub fn random_swap(&mut self) {
        for _ in 0..(self.division.0 * self.division.1 * 5) {
            let tile_a = self.select_random_tile();
            let tile_b = self.select_random_tile();

            self.swap_tile(tile_a, tile_b);
        }
    }

    pub fn random_rotate(&mut self) {
        let mut rng = rand::thread_rng();

        for _ in 0..(self.division.0 * self.division.1 * 5) {
            let tile = self.select_random_tile_without_upper_left();
            let rotate = rng.gen();

            self.rotate_tile(tile, rotate);
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum RotateDirection {
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Distribution<RotateDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RotateDirection {
        match rng.gen_range(0..3) {
            0 => RotateDirection::Rotate90,
            1 => RotateDirection::Rotate180,
            2 => RotateDirection::Rotate270,
            _ => unreachable!(),
        }
    }
}

fn generate_division_size() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(MIN_DIVISION_SIZE..=MAX_DIVISION_SIZE)
}

fn generate_division(division_size: u32) -> (u32, u32) {
    let mut rng = rand::thread_rng();

    let min_division = std::cmp::max(MIN_IMAGE_SIZE / division_size, MIN_DIVISION);
    let max_division = std::cmp::min(MAX_IMAGE_SIZE / division_size, MAX_DIVISION);

    let x = rng.gen_range(min_division..=max_division);
    let y = rng.gen_range(min_division..=max_division);
    (x, y)
}

fn generate_select_limit() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(MIN_SELECT_LIMIT..=MAX_SELECT_LIMIT)
}

fn generate_cost_rate() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(MIN_COST_RATE..=MAX_COST_RATE)
}
