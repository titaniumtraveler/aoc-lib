use self::{
    add_test::{AddTest, AddTestError},
    fetch::{Fetch, FetchError},
    run::Run,
};
use crate::day::Runner;
use clap::{Parser, Subcommand};
use thiserror::Error;

mod add_test;
mod fetch;
mod run;
mod store;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Error)]
pub enum CliError<E> {
    #[error("")]
    FetchError(FetchError),
    #[error("")]
    RunError(E),
    #[error("")]
    AddTestError(AddTestError),
}

impl Cli {
    pub fn run<R: Runner>(&self, runner: &mut R) -> Result<(), CliError<R::Error>> {
        use CliError::*;
        match &self.command {
            Commands::Fetch(fetch) => fetch.run().map_err(FetchError),
            Commands::Run(run) => run.run(runner),
            Commands::AddTest(add_test) => add_test.run().map_err(AddTestError),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Fetch(Fetch),
    Run(Run),
    AddTest(AddTest),
}
