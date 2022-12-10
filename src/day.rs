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
            $($date => Ok(<$day>::run($config,$str)?),)*
            _ => unreachable!("Day {} isn't registered", $config.day),
        }

    };
}

pub trait Day<'a> {
    type Parser;
    type Error: Error;
    fn parse(input: &'a str) -> Result<Self::Parser, Self::Error>;
    fn part_1(parser: Self::Parser) -> Result<u32, Self::Error>;
    fn part_2(parser: Self::Parser) -> Result<u32, Self::Error>;
    fn run(config: Config, str: &'a str) -> Result<u32, Self::Error> {
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
