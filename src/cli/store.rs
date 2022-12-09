use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Store<'a> {
    #[serde(flatten)]
    years: HashMap<u32, HashMap<u8, Day<'a>>>,
    cookie: &'a str,
}

impl<'a> Store<'a> {
    pub fn cookie(&self) -> &'a str {
        self.cookie
    }

    pub fn get_input(&self, year: u32, day: u8, name: &str) -> Option<&Input<'a>> {
        self.years
            .get(&year)
            .and_then(|map| map.get(&day))
            .and_then(|map| map.input.get(name))
    }

    pub fn get_or_insert<F, E>(
        &mut self,
        year: u32,
        day: u8,
        name: impl Into<Cow<'a, str>>,
        f: F,
    ) -> Result<&Input<'a>, E>
    where
        F: FnOnce() -> Result<String, E>,
    {
        match self
            .years
            .entry(year)
            .or_default()
            .entry(day)
            .or_default()
            .input
            .entry(name.into())
        {
            Entry::Occupied(occupied) => Ok(occupied.into_mut()),
            Entry::Vacant(vacant) => {
                let value = Input::new(f()?);
                Ok(vacant.insert(value))
            }
        }
    }

    pub fn insert(
        &mut self,
        year: u32,
        day: u8,
        name: impl Into<Cow<'a, str>>,
        value: Input<'a>,
    ) -> Option<Input> {
        self.years
            .entry(year)
            .or_default()
            .entry(day)
            .or_default()
            .input
            .insert(name.into(), value)
    }

    pub fn entry_input(
        &mut self,
        year: u32,
        day: u8,
        name: impl Into<Cow<'a, str>>,
    ) -> Entry<Cow<'a, str>, Input<'a>> {
        self.years
            .entry(year)
            .or_default()
            .entry(day)
            .or_default()
            .input
            .entry(name.into())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Day<'a> {
    #[serde(flatten)]
    input: HashMap<Cow<'a, str>, Input<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input<'a> {
    input: Cow<'a, str>,
    pub solution1: Option<u32>,
    pub solution2: Option<u32>,
}

impl<'a> Input<'a> {
    pub fn new(input: impl Into<Cow<'a, str>>) -> Self {
        let input = input.into();
        Self {
            input,
            solution1: None,
            solution2: None,
        }
    }
    pub fn input(&self) -> &str {
        self.input.as_ref()
    }
}
