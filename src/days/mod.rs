use crate::parser::MyErr;
use crate::Instant;
use nom::IResult;
use std::fmt::Display;
use std::fs::read_to_string;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub trait Day {
  type Input;

  fn parse(input_string: &str) -> IResult<&str, Self::Input>;

  type Output1: Display;

  fn part_1(input: &Self::Input) -> Self::Output1;

  type Output2: Display;

  fn part_2(input: &Self::Input) -> Self::Output2;

  fn parse_file(fp: &str) -> Result<Self::Input, MyErr> {
    let input_string = read_to_string(fp)?;
    let (_, input) = Self::parse(&input_string)?;
    Ok(input)
  }

  fn run_day(fp: &str) {
    match Self::parse_file(fp) {
      Err(e) => println!("{:?}", e),
      Ok(input) => {
        let before1 = Instant::now();
        println!("Part 1: {}", Self::part_1(&input));
        println!("Part 1 took {}s", before1.elapsed().as_secs_f32());
        let before2 = Instant::now();
        println!("Part 2: {}", Self::part_2(&input));
        println!("Part 2 took {}s", before2.elapsed().as_secs_f32());
      }
    }
  }
}
