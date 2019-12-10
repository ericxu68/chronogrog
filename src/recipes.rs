use string_builder::Builder;

use serde::{Serialize, Deserialize};

use super::phases::PhaseInstanceSpec;
use super::phases::PhaseInstance;

use super::util::get_space_indent;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RecipeSpec {
    pub name: String,

    #[serde(rename="color")]
    pub color_hex: String,

    #[serde(rename="phases")]
    pub phase_specs: Vec<PhaseInstanceSpec>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Recipe {
    pub id: usize,
    pub name: String,
    pub color: String,
    pub phases: Vec<PhaseInstance>
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
