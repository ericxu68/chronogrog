use string_builder::Builder;

use serde::{Serialize, Deserialize};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError};

use super::phases::PhaseInstanceSpec;
use super::phases::PhaseInstance;

use super::util::{get_space_indent, get_naive_date_time_from_string};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]

/// A specification for constructing instances of [Recipe](chronogrog::Recipe).
///
/// The specifications are translated from JSON in the form of a `recipes` block into actual
/// `Recipe` instances. The `Recipe`s themselves can then be transformed into PLA format.
///
pub struct RecipeSpec {
    pub name: String,

    #[serde(rename="color")]
    pub color_hex: String,

    #[serde(rename="phases")]
    pub phase_specs: Vec<PhaseInstanceSpec>,

    #[serde(rename="start")]
    pub start_string: Option<String>
}

impl RecipeSpec {
    /// Retrieve the start date of this `Recipe`, as a `NaiveDateTime`, if it can be parsed from
    /// the input string.
    ///
    /// # Returns
    /// * A `Result` containing either a `NaiveDateTime`, if one can be parsed from the
    ///   deserialized string `start_string`, or a `ParseError` that lets the client know why the
    ///   parsing failed.
    ///
    pub fn start_date(&self) -> Result<NaiveDateTime, ParseError> {
        match &self.start_string {
            Some(x) => get_naive_date_time_from_string(&x[..]),
            None => Ok(NaiveDateTime::new(NaiveDate::from_ymd(1970, 1, 1), NaiveTime::from_hms(0, 0, 0)))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Recipe {
    pub id: usize,
    pub name: String,
    pub color: String,
    pub phases: Vec<PhaseInstance>,
    pub start_date: NaiveDateTime
}

impl Recipe {
    pub fn get_phase_iterator(&self) -> std::slice::Iter<PhaseInstance> {
        self.phases.iter()
    }

    pub fn get_string_in_pla_format(&self, initial_indent: usize) -> String {
        let mut builder: Builder = Builder::default();
        builder.append(format!("[{}] {}\n", self.id, self.name));

        for next_phase in self.get_phase_iterator() {
            builder.append(format!("{}child {}\n", get_space_indent(initial_indent), next_phase.id));
        }

        builder.append("\n");

        builder.string().unwrap()
    }
}
