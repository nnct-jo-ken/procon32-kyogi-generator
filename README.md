# Slide Puzzle Generator

> Generate shuffle puzzle problem

## Install

1. Install Rust and Cargo (See: [rustup.rs](https://rustup.rs/))
2. Install this binary with `cargo install --git https://github.com/procon32/slide-puzzle-generator.git`

## Usage

```
$ spg --help
Shuffle Puzzle Generator 1.0
Shuntaro Nishizawa <me@shun.technology>
Generate shuffle puzzle problem

USAGE:
    slide-puzzle-generator [OPTIONS] <source_name> <problem_name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --division-height <division_height>    number of vertical divisions
        --division-size <division_size>        size of divided image
        --division-width <division_width>      number of horizontal divisions
        --select-limit <select_limit>          limit of selection
        --select-cost-rate <select_rate>       cost rate of select
        --swap-cost-rate <swap_rate>           cost rate of swap

ARGS:
    <source_name>     source file name
    <problem_name>    problem file name
```
