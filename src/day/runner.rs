use std::error::Error;

use super::Config;

pub trait Runner {
    type Error: Error;
    fn run(&mut self, config: Config, str: &str) -> Result<u32, Self::Error>;
}
