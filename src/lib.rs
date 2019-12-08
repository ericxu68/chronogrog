extern crate tint;
use tint::Color;

extern crate chrono;
use chrono::{NaiveDate, Duration};
use chrono::format::ParseError;

use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::default::Default;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

#[derive(Serialize, Deserialize, Clone)]
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

    #[serde(rename="defaultDuration")]
    #[serde(default = "String::new")]
    default_duration: String
}

impl ProductionPhaseTemplate {
    pub fn default_duration(&self) -> Option<Duration> {
        let mut characters: Vec<_> = self.default_duration.chars().collect();
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

    #[serde(rename="phaseTemplates")]
    pub phase_templates: Vec<ProductionPhaseTemplate>,
    pub resources: Vec<Resource>,

    // #[serde(skip_serializing, skip_deserializing)]
    // recipes: Vec<Recipe>,

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
                x.last_id_used = 0;
                x
            },
            Err(e) => {
                panic!("Unable to parse due to: {}", e);
            }
        }
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

    pub fn get_recipe_by_name(&mut self, name: &str) -> Option<Recipe> {
        let mut spec: Option<RecipeSpec> = None;

        let recipes = self.recipe_specs.clone();
        for next_recipe in recipes {
            if next_recipe.name == name {
                spec = Some(next_recipe);
                break;
            }
        }

        match spec {
            Some(x) => Some(Recipe {
                         id: self.get_next_id(),
                         name: x.name.clone(),
                         color: x.color(),
                         phases: vec![]
                     }),
            None => None
        }
    }

    fn get_next_id(&mut self) -> usize {
        let ret_id: usize = self.last_id_used;
        self.last_id_used = self.last_id_used + 1;

        ret_id
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PhaseInstanceSpec {
    template: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RecipeSpec {
    pub name: String,

    #[serde(rename="color")]
    pub color_hex: String,

    #[serde(rename="phases")]
    pub phase_specs: Vec<PhaseInstanceSpec>
}

impl RecipeSpec {
    pub fn color(&self) -> Color {
        Color::from(&self.color_hex)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PhaseInstance {
    duration: Duration
}

#[derive(Clone, PartialEq, Debug)]
pub struct Recipe {
    pub id: usize,
    pub name: String,
    pub color: Color,
    pub phases: Vec<PhaseInstance>
}
