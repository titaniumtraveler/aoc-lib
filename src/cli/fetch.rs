use std::{collections::hash_map::Entry, num::TryFromIntError};

use crate::cli::store::{Input, Store};
use chrono::{Datelike, Local};
use clap::Args;
use reqwest::blocking::Client;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct Fetch {
    #[arg(short, long, default_value_t = false)]
    force: bool,
    #[arg(short, long, default_value_t = true)]
    ignore_existing: bool,
    day: Option<u8>,
    year: Option<u32>,
}

impl Fetch {
    pub fn run<'a>(&self, store: &mut Store<'a>) -> Result<(), FetchError> {
        let now = Local::now();

        let year = match self.year {
            Some(year) => year,
            None => now.year().try_into()?,
        };
        let day = match self.day {
            Some(day) => day,
            None => now.day().try_into()?,
        };

        let cookie = store.cookie();

        if !self.force && self.ignore_existing {
            Self::fetch(store, year, day)?;
            Ok(())
        } else if !self.force && !self.ignore_existing {
            match store.entry_input(year, day, "input") {
                Entry::Vacant(vacant) => {
                    vacant.insert(Input::new(Self::fetch_data(year, day, cookie)?));
                    Ok(())
                }
                Entry::Occupied(_) => Err(FetchError::AlreadyExists { year, day }),
            }
        } else {
            store.insert(
                year,
                day,
                "input",
                Input::new(Self::fetch_data(year, day, cookie)?),
            );
            Ok(())
        }
    }

    pub fn fetch<'a: 'b, 'b>(
        store: &'b mut Store<'a>,
        year: u32,
        day: u8,
    ) -> Result<&'b Input<'a>, reqwest::Error> {
        let cookie = store.cookie();
        store.get_or_insert(year, day, "input", || Self::fetch_data(year, day, cookie))
    }

    pub fn fetch_data(year: u32, day: u8, cookie: &str) -> Result<String, reqwest::Error> {
        Client::new()
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header("Cookie", cookie)
            .send()?
            .text()
    }
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("failed to get valid date {0}")]
    InvalidTime(#[from] TryFromIntError),
    #[error("the entry from the {day}. 12. {year} already exists")]
    AlreadyExists { year: u32, day: u8 },
    #[error(r#"failed to fetch data from "adventofcode.com": {0}"#)]
    ReqwestError(#[from] reqwest::Error),
}
