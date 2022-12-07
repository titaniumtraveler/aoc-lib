use crate::day::Day;
use std::convert::Infallible;

pub struct Todo;

impl Day for Todo {
    type Parser<'a> = ();
    type Error = Infallible;

    fn parse(_str: &str) -> Result<(), Infallible> {
        todo!()
    }

    fn part_1(_parser: Self::Parser<'_>) -> Result<u32, Infallible> {
        todo!()
    }

    fn part_2(_parser: Self::Parser<'_>) -> Result<u32, Infallible> {
        todo!()
    }
}
