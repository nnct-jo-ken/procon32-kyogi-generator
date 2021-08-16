# Slide Puzzle Generator

> Generate shuffle puzzle problem

## Install

1. Install Rust and Cargo (See: [rustup.rs](https://rustup.rs/))
2. Clone this repository
2. Install this binary with `cargo install --path <repository_path>`

## Usage

```
$ spg --help
spg 0.1.2

Generate shuffle puzzle problem

USAGE:
    spg [OPTIONS] <source_name> <problem_name>

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
