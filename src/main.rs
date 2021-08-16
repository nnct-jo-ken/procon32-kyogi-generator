mod fetch;
mod puzzle;

use clap::{crate_authors, crate_version, App, Arg};
use puzzle::Puzzle;
use std::fs;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Shuffle Puzzle Generator")
        .name("spg")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generate shuffle puzzle problem")
        .arg(
            Arg::with_name("division_size")
                .long("division-size")
                .help("size of divided image")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("division_width")
                .long("division-width")
                .help("number of horizontal divisions")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("division_height")
                .long("division-height")
                .help("number of vertical divisions")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("select_limit")
                .long("select-limit")
                .help("limit of selection")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("select_rate")
                .long("select-cost-rate")
                .help("cost rate of select")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("swap_rate")
                .long("swap-cost-rate")
                .help("cost rate of swap")
                .takes_value(true)
                .use_delimiter(false),
        )
        .arg(
            Arg::with_name("source_name")
                .help("source file name")
                .required(true),
        )
        .arg(
            Arg::with_name("problem_name")
                .help("problem file name")
                .required(true),
        )
        .get_matches();

    let division_size = matches
        .value_of("division_size")
        .and_then(|x| x.parse::<u32>().ok());
    let division = {
        let division_width = matches
            .value_of("division_width")
            .and_then(|x| x.parse::<u32>().ok());
        let division_height = matches
            .value_of("division_height")
            .and_then(|x| x.parse::<u32>().ok());
        match (division_width, division_height) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    };
    let select_limit = matches
        .value_of("select_limit")
        .and_then(|x| x.parse::<u32>().ok());
    let select_rate = matches
        .value_of("select_rate")
        .and_then(|x| x.parse::<u32>().ok());
    let swap_rate = matches
        .value_of("swap_rate")
        .and_then(|x| x.parse::<u32>().ok());

    let source_name = matches.value_of("source_name").unwrap();
    let problem_name = matches.value_of("problem_name").unwrap();

    let mut puzzle = Puzzle::generate(
        division_size,
        division,
        select_limit,
        select_rate,
        swap_rate,
    )?;

    let mut source_file = BufWriter::new(fs::File::create(source_name)?);
    let mut problem_file = BufWriter::new(fs::File::create(problem_name)?);

    puzzle.decode(&mut source_file)?;

    puzzle.random_swap();
    puzzle.random_rotate();

    puzzle.decode(&mut problem_file)?;

    Ok(())
}
