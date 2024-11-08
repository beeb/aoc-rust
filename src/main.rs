use std::{fs, ops::RangeInclusive};

use anyhow::{bail, Context as _, Result};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};

#[allow(clippy::wildcard_imports)]
use days::*;

mod days;

const YEAR: usize = 2024; // change this if needed
const CLI_DAY_RANGE: RangeInclusive<i64> = 1..=25;
const VALID_DAY_RANGE: RangeInclusive<u32> =
    (*CLI_DAY_RANGE.start() as u32)..=(*CLI_DAY_RANGE.end() as u32);

/// Advent of Code
#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the code for one or all days
    Run {
        #[arg(
            value_parser = clap::value_parser!(u32).range(CLI_DAY_RANGE),
            help = "The number of the day you want to run (1-25)")
        ]
        day: Option<u32>,
        #[arg(short, long, help = "Runs all days sequentially")]
        all: bool,
    },
    /// Get the input file for one or all days
    Get {
        #[arg(
            value_parser = clap::value_parser!(u32).range(CLI_DAY_RANGE),
            help = "The number of the day you want to get the input for (1-25)")
        ]
        day: Option<u32>,
        #[arg(short, long, help = "Downloads input for all days sequentially")]
        all: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { day, all } => {
            if all {
                return run_all_days();
            }
            if let Some(day) = day {
                return run_day(day);
            }
            println!("No day parameter specified, attempting to run today's code");
            let now_day = get_today()?;
            println!("Running day {now_day}");
            run_day(now_day)
        }
        Commands::Get { day, all } => {
            if all {
                return download_all_inputs();
            }
            if let Some(day) = day {
                return download_input(day);
            }
            println!("No day parameter specified, attempting to download today's input");
            let now_day = get_today()?;
            println!("Getting input for day {now_day}");
            download_input(now_day)
        }
    }
}

fn get_today() -> Result<u32> {
    let now = Local::now();
    let now_day = now.day();
    if now.month() == 12 && VALID_DAY_RANGE.contains(&now_day) {
        Ok(now_day)
    } else {
        bail!("Today is not a valid Advent of Code day. Please specify a day");
    }
}

#[allow(const_item_mutation)]
fn run_all_days() -> Result<()> {
    VALID_DAY_RANGE.try_for_each(run_day)
}

fn run_day(day: u32) -> Result<()> {
    println!("======== DAY {day} ========");
    // I'd like to do this with a macro, but I'm not sure how to do it.
    let input_file = &format!("inputs/day{day:02}.txt");
    match day {
        1 => day01::Day01::run_day(input_file),
        2 => day02::Day02::run_day(input_file),
        3 => day03::Day03::run_day(input_file),
        4 => day04::Day04::run_day(input_file),
        5 => day05::Day05::run_day(input_file),
        6 => day06::Day06::run_day(input_file),
        7 => day07::Day07::run_day(input_file),
        8 => day08::Day08::run_day(input_file),
        9 => day09::Day09::run_day(input_file),
        10 => day10::Day10::run_day(input_file),
        11 => day11::Day11::run_day(input_file),
        12 => day12::Day12::run_day(input_file),
        13 => day13::Day13::run_day(input_file),
        14 => day14::Day14::run_day(input_file),
        15 => day15::Day15::run_day(input_file),
        16 => day16::Day16::run_day(input_file),
        17 => day17::Day17::run_day(input_file),
        18 => day18::Day18::run_day(input_file),
        19 => day19::Day19::run_day(input_file),
        20 => day20::Day20::run_day(input_file),
        21 => day21::Day21::run_day(input_file),
        22 => day22::Day22::run_day(input_file),
        23 => day23::Day23::run_day(input_file),
        24 => day24::Day24::run_day(input_file),
        25 => day25::Day25::run_day(input_file),
        d => bail!("provided unsupported day {d}"),
    }
}

#[allow(const_item_mutation)]
fn download_all_inputs() -> Result<()> {
    VALID_DAY_RANGE.try_for_each(download_input)
}

fn download_input(day: u32) -> Result<()> {
    // Read session cookie from .session file
    let session = fs::read_to_string(".session").context("reading .session file")?;
    let session = session.trim();
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={session};"))
        .send().context("sending HTTP request to download input")?
        .error_for_status()
        .with_context(|| format!("retrieving the input for day {day}. Do you have the correct session cookie in the .session file?"))?;

    let text = response.text().context("decoding response body as text")?;
    let path = format!("inputs/day{day:02}.txt");
    fs::write(&path, text).context("writing input to file")?;
    println!("Successfully downloaded input to {}", &path);
    Ok(())
}
