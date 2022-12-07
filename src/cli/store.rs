use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct Store<'a> {
    #[serde(flatten)]
    years: HashMap<u32, HashMap<u8, Day<'a>>>,
}

impl<'a> Store<'a> {
    fn get_input(&self, year: u32, day: u8, name: &str) -> Option<&Input<'a>> {
        self.years
            .get(&year)
            .and_then(|map| map.get(&day))
            .and_then(|map| map.input.get(name))
    }

    fn get_or_insert(&mut self, year: u32, day: u8, name: Cow<'a, str>) {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day<'a> {
    #[serde(flatten)]
    input: HashMap<Cow<'a, str>, Input<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input<'a> {
    input: Cow<'a, str>,
    solution_1: Option<u32>,
    solution_2: Option<u32>,
}
