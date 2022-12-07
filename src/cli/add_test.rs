use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct AddTest {
    #[arg(short, long)]
    name: String,
    #[arg(value_enum)]
    day: Option<u8>,
    year: Option<u32>,
}

impl AddTest {
    pub fn run(&self) -> Result<(), AddTestError> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum AddTestError {}
