mod parser;
use chrono::prelude::*;
use clap::{Parser, Subcommand};
use days::*;
use std::fs;
use std::time::Instant;

mod days;

const YEAR: usize = 2022;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(value_name = "DAY", help = "The number of the day you want to run")]
        day: Option<String>,
        #[arg(short, long, help = "Runs all days sequentially")]
        all: bool,
    },
    GetInput {
        #[arg(
            value_name = "DAY",
            help = "The number of the day you want to get the input for"
        )]
        day: Option<String>,
        #[arg(short, long, help = "Downloads input for all days sequentially")]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { day, all } => {
            if *all {
                run_all_days();
            } else {
                match day {
                    Some(day) => run_day(parse_day(day)),
                    None => {
                        println!("No day parameter specified, attempting to run today");
                        let now_day = get_today();
                        println!("Running day {}", now_day);
                        run_day(now_day);
                    }
                }
            }
        }
        Commands::GetInput { day, all } => {
            if *all {
                download_all_input();
            } else {
                match day {
                    Some(day) => download_input(parse_day(day)),
                    None => {
                        println!("No day parameter specified, ttempting to download today's input");
                        let now_day = get_today();
                        println!("Getting input for day {}", now_day);
                        download_input(now_day);
                    }
                }
            }
        }
    }
}

fn get_today() -> usize {
    let now = Local::now();
    let now_day = now.day();
    if now.month() == 12 && (1..=25).contains(&now_day) {
        now_day.try_into().unwrap()
    } else {
        panic!("Today is not a valid Advent of Code day. Please specify a day");
    }
}

fn parse_day(day: &str) -> usize {
    match day.parse() {
        Ok(i) => {
            if (i..=25).contains(&i) {
                i
            } else {
                panic!("{} is not a valid day. Only days 1-25 are allowed.", i)
            }
        }
        Err(_) => {
            panic!("{} is not a valid day. Please provide a number.", day)
        }
    }
}

fn run_all_days() {
    (1..=25).map(run_day).collect()
}
// Panics if you provide a value outside the range of 1 to 25
fn run_day(day: usize) {
    println!("======== DAY {} ========", day);
    // I'd like to do this with a macro, but I'm not sure how to do it.
    let input_fp = &format!("inputs/day{:02}.txt", day);
    match day {
        1 => day01::Day01::run_day(input_fp),
        2 => day02::Day02::run_day(input_fp),
        3 => day03::Day03::run_day(input_fp),
        4 => day04::Day04::run_day(input_fp),
        5 => day05::Day05::run_day(input_fp),
        6 => day06::Day06::run_day(input_fp),
        7 => day07::Day07::run_day(input_fp),
        8 => day08::Day08::run_day(input_fp),
        9 => day09::Day09::run_day(input_fp),
        10 => day10::Day10::run_day(input_fp),
        11 => day11::Day11::run_day(input_fp),
        12 => day12::Day12::run_day(input_fp),
        13 => day13::Day13::run_day(input_fp),
        14 => day14::Day14::run_day(input_fp),
        15 => day15::Day15::run_day(input_fp),
        16 => day16::Day16::run_day(input_fp),
        17 => day17::Day17::run_day(input_fp),
        18 => day18::Day18::run_day(input_fp),
        19 => day19::Day19::run_day(input_fp),
        20 => day20::Day20::run_day(input_fp),
        21 => day21::Day21::run_day(input_fp),
        22 => day22::Day22::run_day(input_fp),
        23 => day23::Day23::run_day(input_fp),
        24 => day24::Day24::run_day(input_fp),
        25 => day25::Day25::run_day(input_fp),
        d => panic!("Provided unsupported day {}", d),
    }
}

fn download_all_input() {
    (1..=25).map(download_input).collect()
}

fn download_input(day: usize) {
    // Read session cookie from .session file
    let session = fs::read_to_string(".session").expect("Could not find .session file");
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={};", session))
        .send()
        .unwrap();

    if response.status().is_success() {
        let mut text = response.text().unwrap();
        // Remove trailing newline
        text.pop();
        let path = format!("inputs/day{:02}.txt", day);
        fs::write(&path, text).unwrap();
        println!("Successfully downloaded input to {}", &path);
    } else {
        panic!(
            "Could not get input for day {}. Is your correct session cookie in your .session file?",
            day
        )
    }
}
