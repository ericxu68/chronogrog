use chrono::Duration;

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

pub fn get_duration_in_hours(duration: Duration) -> i64 {
    duration.num_hours()
}

pub fn get_space_indent(indents: usize) -> String {
    " ".repeat(indents * 2)
}
