extern crate chrono;
use chrono::{NaiveDate, Duration};
use chrono::format::ParseError;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::default::Default;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct ProductionPhase {
    pub description: String,
    pub id: usize,
    pub order: usize,

    #[serde(rename="defaultDuration")]
    #[serde(default = "String::new")]
    default_duration: String
}

// pub struct Resource {
//     pub id: usize,
//     pub name: String,
//     pub
//
// }

impl ProductionPhase {
    pub fn default_duration(&self) -> Option<Duration> {
        let mut characters: Vec<_> = self.default_duration.chars().collect();
        match characters.pop() {
            Some(x) => {
                let digit_string: String = characters.into_iter().collect();
                let digits: i64 = digit_string.parse::<i64>().unwrap();
                match x {
                    'm' => Some(Duration::days(digits*30)),
                    'w' => Some(Duration::weeks(digits)),
                    'd' => Some(Duration::days(digits)),
                    'h' => Some(Duration::hours(digits)),
                    _ => Some(Duration::days(digits))
                }
            },
            None => None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductionTimeline {
    pub configuration: String,
    pub start: String
}

impl ProductionTimeline {
    pub fn start_date(&self) -> std::result::Result<NaiveDate, ParseError> {
        NaiveDate::parse_from_str(self.start.as_str(), "%Y-%m-%d")
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductionSchedule {
    pub name: String,
    pub id: usize,
    pub timeline: ProductionTimeline,
    pub phases: Vec<ProductionPhase>
}

impl ProductionSchedule {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents: String = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let result: Result<ProductionSchedule> = serde_json::from_str(contents.as_str());
        match result {
            Ok(x) => x,
            Err(e) => {
                panic!("Unable to parse due to: {}", e);
            }
        }
    }
}
