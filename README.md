Largely copied from https://github.com/RikvanToor/aoc-rust-template, upgraded for `clap` v4

# Advent of Code Rust template

This is a basic template you can fork and implement your Advent of Code solutions in. `src/days/` contains a source file for all 25 days, where an implementation of a trait `Day` should be written.

```rust
pub trait Day {
  type Input;

  fn parse(input_string: &str) -> IResult<&str, Self::Input>;

  type Output1: Display;

  fn part_1(input: &Self::Input) -> Self::Output1;

  type Output2: Display;

  fn part_2(input: &Self::Input) -> Self::Output2;
}
```

The parsers should be written using [nom](https://docs.rs/nom/latest/nom/).

## Usage

First, find your session cookie. To do so, go to adventofcode.com, log in, and press F12. In the network tab, click any request, and find your session cookie in the request headers. Paste it into `.session`. You can now donload input files. To do so, run the subcommand `get-input`: `cargo run -- get-input 1`. The `1` stands for day 1, meaning it will download the input of day 1 of Advent of Code 2021. Alternatively, from december 1st to 25th, you can skip the day parameter, and the program will download today's input. You can also use `--all` instead of a day parameter to download all input files.

To run your implementation, use `cargo run -- run 1` to run day 1. Just like `get-input`, you can skip the day parameter to run today's program, or use `--all` to run all days.
