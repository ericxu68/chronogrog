use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use chrono::{Duration, NaiveDate, NaiveDateTime, ParseError};

pub fn convert_string_to_duration(duration_string: &str) -> Option<Duration> {
    let mut characters: Vec<_> = duration_string.chars().collect();
    let mut identifier = None;
    if characters.len() > 0 {
        let last_character = characters[characters.len()- 1];
        let is_last_character_digit = last_character.to_string().parse::<usize>().is_ok();
        identifier = Some('d');
        if !is_last_character_digit {
            identifier = characters.pop();
        }
    }

    match identifier {
        Some(x) => {
            let digit_string: String = characters.into_iter().collect();
            let digits: i64 = digit_string.parse::<i64>().unwrap();
            match x {
                'm' => Some(Duration::days(digits*30)),
                'w' => Some(Duration::weeks(digits)),
                'd' => Some(Duration::days(digits)),
                'h' => Some(Duration::hours(digits)),
                _ => None
            }
        },
        None => None
    }
}

/// Try to convert a `String` to a `NativeDateTime`.
///
/// # Arguments
/// * `date_string`: A string slice containing either a `NaiveDateTime` in `YYYY-MM-DD HH:MM:SS`
///   format, or a `NaiveDate` in `YYYY-MM-DD` format.
///
/// # Returns
/// * A `Result` containing either:
///   - A `NaiveDateTime`, if one can be constructed from the given `&str`, or
///   - A `ParseError` containing an explanation as to why the `NaiveDateTime` could not be
///     constructed.
///
/// # Notes
/// - If a `NaiveDate` is given (the string is in `YYYY-MM-DD` format instead of
///   `YYYY-MM-DD HH:MM:SS`), then this will be converted to a `NaiveDateTime` at `00:00:00`.
///
pub fn get_naive_date_time_from_string(date_string: &str) -> Result<NaiveDateTime, ParseError> {
    match NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S") {
        Ok(x) => Ok(x),
        Err(e) => {
            if e.description() == "premature end of input" {
                match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
                    Ok(x) => Ok(x.and_hms(0, 0, 0)),
                    Err(e) => Err(e)
                }
            } else {
                Err(e)
            }
        }
    }
}

pub fn get_duration_in_hours(duration: Duration) -> i64 {
    duration.num_hours()
}

pub fn get_space_indent(indents: usize) -> String {
    " ".repeat(indents * 2)
}

pub fn get_json_data_from_file(in_filename: &str) -> std::io::Result<String> {
    let file = File::open(in_filename)?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
