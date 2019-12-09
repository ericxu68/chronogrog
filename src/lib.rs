use std::default::Default;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;

extern crate tint;
use tint::Color;

extern crate chrono;
use chrono::{NaiveDate, Duration};
use chrono::format::ParseError;

use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

extern crate string_builder;
use string_builder::Builder;

mod util;

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceType {
    Fermentor,
    Kettle,
    MashTun,
    LauterTun,
    Keg,
    Kegerator,
    Other(String)
}

impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let s = String::deserialize(deserializer)?;
            Ok(match s.as_str() {
                "fermentor" => ResourceType::Fermentor,
                "kettle" => ResourceType::Kettle,
                "mashtun" => ResourceType::MashTun,
                "lautertun" => ResourceType::LauterTun,
                "keg" => ResourceType::Keg,
                "kegerator" => ResourceType::Kegerator,
                _ => ResourceType::Other(s)
            })
    }
}

impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        serializer.serialize_str(match *self {
            ResourceType::Fermentor => "fermentor",
            ResourceType::Kettle => "kettle",
            ResourceType::MashTun => "mashtun",
            ResourceType::LauterTun => "lautertun",
            ResourceType::Keg => "keg",
            ResourceType::Kegerator => "kegerator",
            ResourceType::Other(ref other) => other
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Resource {
    pub id: usize,
    pub name: String,

    #[serde(rename="type")]
    pub resource_type: ResourceType,

    #[serde(rename="capacity")]
    pub capacity_str: String
}

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
        util::convert_string_to_duration(&self.default_duration[..])
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PhaseInstanceSpec {
    #[serde(default = "String::new")]
    description: String,

    template: String,

    #[serde(rename = "duration")]
    #[serde(default = "String::new")]
    duration_string: String
}

impl PhaseInstanceSpec {
    pub fn duration(&self) -> Option<Duration> {
        match self.duration_string.is_empty() {
            true => None,
            false => util::convert_string_to_duration(&self.duration_string[..])
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RecipeSpec {
    pub name: String,

    #[serde(rename="color")]
    pub color_hex: String,

    #[serde(rename="phases")]
    pub phase_specs: Vec<PhaseInstanceSpec>
}

#[derive(Clone, PartialEq, Debug)]
pub struct PhaseInstance {
    pub description: String,
    pub id: usize,
    pub color_hex: String,
    duration: Duration,
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
}

#[derive(Serialize, Deserialize)]
pub struct ProductionSchedule {
    pub name: String,
    pub id: usize,
    pub timeline: ProductionTimeline,

    #[serde(rename="phaseTemplates")]
    pub phase_templates: Vec<ProductionPhaseTemplate>,
    pub resources: Vec<Resource>,

    #[serde(skip_serializing, skip_deserializing)]
    recipes: Vec<Recipe>,

    #[serde(rename="recipes")]
    pub recipe_specs: Vec<RecipeSpec>,

    #[serde(skip_serializing, skip_deserializing)]
    last_id_used: usize
}

impl ProductionSchedule {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents: String = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let result: serde_json::Result<ProductionSchedule> = serde_json::from_str(contents.as_str());
        match result {
            Ok(mut x) => {
                x.init();
                x
            },
            Err(e) => {
                panic!("Unable to parse due to: {}", e);
            }
        }
    }

    pub fn init(&mut self) {
        self.last_id_used = 0;
        self.rebuild_recipes_from_specs();
    }

    pub fn get_phase_by_id(&self, id: &str) -> Option<ProductionPhaseTemplate> {
        for next_phase in &self.phase_templates {
            if next_phase.id == id {
                return Some(next_phase.clone());
            }
        }

        None
    }

    pub fn get_resource_by_id(&self, id: usize) -> Option<Resource> {
        for next_res in &self.resources {
            if next_res.id == id {
                return Some(next_res.clone());
            }
        }

        None
    }

    pub fn get_available_resource_by_type(&self, resource_type: ResourceType) -> Option<Resource> {
        let resources: Vec<Resource> = self.resources.clone();
        if resources.iter().any(|x| x.resource_type == resource_type) {
            return resources.into_iter()
                            .filter(|x| x.resource_type == resource_type)
                            .next();
        }

        None
    }

    // pub fn get_recipe_by_id(&mut self, id: usize) -> Option<Recipe> {
    //     let mut spec: Option<RecipeSpec> = None;
    //     let recipes = self.recipe_specs.clone();
    //     for next_recipe in recipes {
    //         if next_recipe.id == id {
    //             spec = Some(next_recipe);
    //             break;
    //         }
    //     }
    //
    //     match spec {
    //         Some(x) => Some(Recipe {
    //                      id: self.get_next_id(),
    //                      name: x.name.clone(),
    //                      color: x.color(),
    //                      phases: vec![]
    //                  }),
    //         None => None
    //     }
    // }

    pub fn get_recipe_by_name(&self, name: &str) -> Option<Recipe> {
        for next_recipe in &self.recipes {
            if next_recipe.name == name {
                return Some(next_recipe.clone());
            }
        }

        None
    }

    pub fn get_recipe_iterator(&self) -> std::slice::Iter<Recipe> {
        self.recipes.iter()
    }

    pub fn output_to_pla_format(&self) -> String {
        let mut builder = Builder::default();
        for next_recipe in self.get_recipe_iterator() {
            builder.append(format!("[{}] {}\n", next_recipe.id, next_recipe.name));

            for next_phase in next_recipe.get_phase_iterator() {
                builder.append(format!("{}child {}\n", util::get_space_indent(1), next_phase.id));
            }

            builder.append("\n");

            for next_phase in next_recipe.get_phase_iterator() {
                builder.append(format!("{}[{}] {}\n", util::get_space_indent(1), next_phase.id, next_phase.description));
                builder.append(format!("{}color {}\n", util::get_space_indent(2), next_phase.color_hex));
                builder.append(format!("{}duration {}\n", util::get_space_indent(2), util::get_duration_in_hours(next_phase.duration)));
                builder.append("\n");
            }
        }

        let final_pla: String = builder.string().unwrap();

        // Remove the last newline at the end of the file, as it's unnecessary
        final_pla[..final_pla.len() - 1].to_string()
    }

    fn get_next_id(&mut self) -> usize {
        self.last_id_used += 1;

        self.last_id_used
    }

    fn rebuild_recipes_from_specs(&mut self) {
        let mut recipes_vec = vec![];
        let recipes = self.recipe_specs.clone();
        for next_recipe_spec in recipes {
            let mut recipe_template: Recipe = Recipe {
                id: self.get_next_id(),
                name: next_recipe_spec.name.clone(),
                color: next_recipe_spec.color_hex.clone(),
                phases: vec![]
            };

            recipe_template.phases = self.rebuild_phases_from_specs(&next_recipe_spec);

            recipes_vec.push(recipe_template);
        }

        self.recipes = recipes_vec;
    }

    fn rebuild_phases_from_specs(&mut self, recipe_spec: &RecipeSpec) -> Vec<PhaseInstance> {
        let mut phases: Vec<PhaseInstance> = vec![];

        for next_spec in recipe_spec.phase_specs.iter() {
            let id: usize = self.get_next_id();

            // If the duration is specified in the spec, use that duration.
            // Otherwise, use the default duration by looking up from the template.
            let mut dur: Option<Duration> = next_spec.duration();
            dur = match dur {
                Some(x) => Some(x),
                None => {
                    let template: ProductionPhaseTemplate = self.get_phase_by_id(&next_spec.template[..]).unwrap();
                    template.default_duration()
                }
            };

            // If the description is specified in the spec, use that description.
            // Otherwise, use the description by looking up from the template.
            let mut description: String = next_spec.description.clone();
            description = match description.is_empty()  {
                true => {
                    let template: ProductionPhaseTemplate = self.get_phase_by_id(&next_spec.template[..]).unwrap();
                    template.description
                },
                false => description
            };

            phases.push(PhaseInstance {
                description: description,
                id: id,
                color_hex: recipe_spec.color_hex.clone(),
                duration: match dur {
                    Some(x) => x,

                    // Default to a single day if nothing else works
                    None => Duration::days(1)
                }
            });
        }

        phases
    }
}
