use super::Config;

pub trait Runner {
    type Error;
    fn run(&mut self, config: Config, str: &str) -> Result<u32, Self::Error>;
}
