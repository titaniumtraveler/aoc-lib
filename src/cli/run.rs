use crate::day::{Part, Runner};
use clap::Args;
use std::path::PathBuf;

use super::CliError;

#[derive(Debug, Args)]
pub struct Run {
    #[arg(short, long, default_value = "input")]
    input: String,
    #[arg(long)]
    input_file: PathBuf,
    #[arg(value_enum)]
    part: Part,
    day: Option<u8>,
    year: Option<u32>,
}

impl Run {
    pub fn run<R: Runner>(&self, runner: &mut R) -> Result<(), CliError<R::Error>> {
        todo!()
    }
}
