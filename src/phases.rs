use chrono::Duration;

use string_builder::Builder;

use serde::{Serialize, Deserialize};

use super::util::{get_space_indent, get_duration_in_hours, convert_string_to_duration};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct ProductionPhaseTemplate {
    pub description: String,
    pub id: String,
    pub order: usize,

    #[serde(rename="color")]
    #[serde(default = "String::new")]
    color_hex: String,

    #[serde(rename="defaultDuration")]
    #[serde(default = "String::new")]
    default_duration: String
}

impl ProductionPhaseTemplate {
    pub fn default_duration(&self) -> Option<Duration> {
        convert_string_to_duration(&self.default_duration[..])
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PhaseInstanceSpec {
    #[serde(default = "String::new")]
    pub description: String,

    pub template: String,

    #[serde(rename = "duration")]
    #[serde(default = "String::new")]
    pub duration_string: String
}

impl PhaseInstanceSpec {
    pub fn duration(&self) -> Option<Duration> {
        match self.duration_string.is_empty() {
            true => None,
            false => convert_string_to_duration(&self.duration_string[..])
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PhaseInstance {
    pub description: String,
    pub id: usize,
    pub color_hex: String,
    pub duration: Duration,
}

impl PhaseInstance {
    pub fn get_string_in_pla_format(&self) -> String {
        let mut builder = Builder::default();
        builder.append(format!("{}[{}] {}\n", get_space_indent(1), self.id, self.description));
        builder.append(format!("{}color {}\n", get_space_indent(2), self.color_hex));
        builder.append(format!("{}duration {}\n", get_space_indent(2), get_duration_in_hours(self.duration)));
        builder.append("\n");

        builder.string().unwrap()
    }
}
