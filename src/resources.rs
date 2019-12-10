use serde::{Serialize, Deserialize, Serializer, Deserializer};

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
