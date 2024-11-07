use winnow::PResult;

use crate::days::Day;

pub struct Day10;

impl Day for Day10 {
    type Input = String;

    fn parser(_input: &mut &str) -> PResult<Self::Input> {
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
