use crate::day::Part;
use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct Fetch {
    #[arg(short, long)]
    force: bool,
    #[arg(value_enum)]
    part: Part,
    day: Option<u8>,
    year: Option<u32>,
}

impl Fetch {
    pub fn run(&self) -> Result<(), FetchError> {
        todo!()
    }

    fn fetch() {
        todo!()
    }
    fn fetch_data() {}
}

#[derive(Debug, Error)]
pub enum FetchError {}
