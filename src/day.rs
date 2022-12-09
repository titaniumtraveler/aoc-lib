pub use self::{runner::Runner, todo::Todo};
use clap::ValueEnum;
use std::{error::Error, result::Result};

mod runner;
mod todo;

#[macro_export]
macro_rules! calendar {
    (match ($config:expr,$str:expr) {
            $( $date:pat => $day:ty ),*,
    }) => {
        match $config.day {
            $($date => <$day>::run($config,$str)?,)*
            _ => unreachable!("Day {}", $config.day),
        }

    };
}

pub trait Day {
    type Parser<'a>: 'a;
    type Error: Error;
    fn parse(str: &str) -> Result<Self::Parser<'_>, Self::Error>;
    fn part_1(parser: Self::Parser<'_>) -> Result<u32, Self::Error>;
    fn part_2(parser: Self::Parser<'_>) -> Result<u32, Self::Error>;
    fn run(config: Config, str: &str) -> Result<u32, Self::Error> {
        let parser = Self::parse(str)?;
        match config.part {
            Part::Part1 => Self::part_1(parser),
            Part::Part2 => Self::part_2(parser),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    pub year: u32,
    pub day: u8,
    pub part: Part,
}

impl Config {
    pub fn new(year: u32, day: u8, part: Part) -> Self {
        Self { year, day, part }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}
