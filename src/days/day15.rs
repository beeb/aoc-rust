use nom::IResult;

use crate::days::Day;

pub struct Day15;

impl Day for Day15 {
    type Input = String;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        unimplemented!("parser")
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        unimplemented!("part_1")
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
