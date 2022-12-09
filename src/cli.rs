use self::{
    add_test::{AddTest, AddTestError},
    fetch::{Fetch, FetchError},
    run::{Run, RunError},
    store::Store,
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
pub enum CliError<R: Runner> {
    #[error(transparent)]
    FetchError(FetchError),
    #[error(transparent)]
    RunError(RunError<R>),
    #[error(transparent)]
    AddTestError(#[from] AddTestError),
}

impl Cli {
    pub fn run<R: Runner>(&self, runner: &mut R, store: &mut Store) -> Result<(), CliError<R>> {
        use CliError::*;
        match &self.command {
            Commands::Fetch(fetch) => fetch.run(store).map_err(FetchError),
            Commands::Run(run) => run.run(store, runner).map_err(RunError),
            Commands::AddTest(add_test) => add_test.run(store).map_err(AddTestError),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Fetch(Fetch),
    Run(Run),
    AddTest(AddTest),
}
