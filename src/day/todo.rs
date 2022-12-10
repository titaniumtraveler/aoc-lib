use crate::day::Day;
use std::convert::Infallible;

pub struct Todo;

impl<'a> Day<'a> for Todo {
    type Parser = ();
    type Error = Infallible;

    fn parse(_str: &str) -> Result<(), Infallible> {
        todo!()
    }

    fn part_1(_parser: Self::Parser) -> Result<u32, Infallible> {
        todo!()
    }

    fn part_2(_parser: Self::Parser) -> Result<u32, Infallible> {
        todo!()
    }
}
