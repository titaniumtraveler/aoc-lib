use crate::{
    cli::{
        fetch::{Fetch, FetchError},
        store::Input,
        Store,
    },
    day::{Config, Part, Runner},
};
use chrono::{Datelike, Local};
use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct Run {
    #[arg(value_enum)]
    part: Part,
    day: Option<u8>,
    year: Option<u32>,
}

impl Run {
    pub fn run<'a: 'b, 'b, R: Runner<'a>>(
        &self,
        store: &'b mut Store<'a>,
        runner: &'b mut R,
    ) -> Result<(), RunError<'a, R>> {
        let (input, config) = fetch_input_and_config(self, store)?;
        let actual_value = runner.run(config, input.input()).map_err(RunError::Run)?;
        println!("{actual_value}");
        use Part::*;
        if let Some(expected) = match config.part {
            Part1 => input.solution1,
            Part2 => input.solution2,
        } {
            if expected == actual_value {
                Ok(())
            } else {
                Err(RunError::ResultInconsistent {
                    expected,
                    actual_value,
                })
            }
        } else {
            Ok(())
        }
    }
}

fn fetch_input_and_config<'a: 'b, 'b>(
    run: &Run,
    store: &'b mut Store<'a>,
) -> Result<(&'b Input<'a>, Config), FetchError> {
    let now = Local::now();

    let year = match run.year {
        Some(year) => year,
        None => now.year().try_into()?,
    };
    let day = match run.day {
        Some(day) => day,
        None => now.day().try_into()?,
    };

    Ok((
        Fetch::fetch(store, year, day)?,
        Config::new(year, day, run.part),
    ))
}

#[derive(Debug, Error)]
pub enum RunError<'a, R: Runner<'a>> {
    #[error(transparent)]
    Fetch(#[from] FetchError),
    #[error("failed running task: {0}")]
    Run(R::Error),
    #[error("expected return value of {expected} and got {actual_value}")]
    ResultInconsistent { expected: u32, actual_value: u32 },
}
