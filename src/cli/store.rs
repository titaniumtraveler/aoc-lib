use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    mem::transmute,
};
use thiserror::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store<'a> {
    #[serde(flatten)]
    years: HashMap<u32, HashMap<u8, Day<'a>>>,
    pub cookie: Option<&'a str>,
}

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("the cookie is missing: can't access puzzle inputs")]
    Missing,
}

impl<'a> Store<'a> {
    pub fn try_cookie(&self) -> Result<&'a str, CookieError> {
        self.cookie.ok_or(CookieError::Missing)
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
    pub fn input<'b>(&'b self) -> &'a str {
        match &self.input {
            Cow::Borrowed(str) => str,
            // Safety?: I hope that nobody uses this and then invalidates the &'a str?
            //
            // I know that that's unsafe and may lead to undefined behavior,
            // but it's FINE!
            //
            // Also all errors return by the Runner (which are the only reason why I even need this.
            // should be returned before deallocation Store. So all references to the Strings should be
            // gone by then.
            //
            // Just don't run. Keep the errors around. And then replace one of the inputs. Please.
            Cow::Owned(str) => unsafe { transmute::<&'b str, &'a str>(str.as_str()) },
        }
    }
}
