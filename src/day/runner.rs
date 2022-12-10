use std::error::Error;

use super::Config;

pub trait Runner<'a> {
    type Error: Error;
    fn run(&mut self, config: Config, str: &'a str) -> Result<u32, Self::Error>;
}
