use clap::Args;
use thiserror::Error;

use super::store::Store;

#[derive(Debug, Args)]
pub struct AddTest {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    year: Option<u32>,
    #[arg(short, long, default_value_t = false)]
    force: bool,
    #[arg(short, long)]
    solution1: Option<u32>,
    #[arg(short, long)]
    solution2: Option<u32>,
}

impl AddTest {
    pub fn run(&self, store: &mut Store) -> Result<(), AddTestError> {
        todo!("{self:?}, {store:?}")
    }
}

#[derive(Debug, Error)]
pub enum AddTestError {}
