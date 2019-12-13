use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Clone, Debug, PartialEq)]

/// Type of a particular resource.
pub enum ResourceType {
    /// A unit for storage during fermentation.
    Fermentor,

    /// A resource for heating water and boiling sweet wort.
    Kettle,

    /// A place to convert raw grain into sweet wort.
    MashTun,

    /// A place for separating the liquid and solid components of a mash. In homebrewing, this is
    /// often synonymous with a hot liquor tank, which is a place for holding hot water.
    LauterTun,

    /// A place for carbonating, aging, and serving beer.
    Keg,

    /// A place to put kegs in order to refrigerate.
    Kegerator,

    /// A resource type that has not yet been added to the standard enum. The "real" type of the
    /// resource, for the purposes of serialization and deserialization, will be contained in the
    /// string variable present in the enum instance.
    Other(String)
}

impl From<&str> for ResourceType {
    /// Convert from a string slice (`&str`) to a `ResourceType`.
    ///
    /// # Arguments
    /// * `res`: A string slice (`&str`) that will be converted.
    ///
    /// # Returns
    ///
    /// * A `ResourceType` corresponding to the appropriate `&str` reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chronogrog::resources::ResourceType;
    /// let s = "fermentor";
    ///
    /// assert_eq!(ResourceType::Fermentor, ResourceType::from(s));
    /// ```
    fn from(res: &str) -> Self {
        match res {
            "fermentor" => ResourceType::Fermentor,
            "kettle" => ResourceType::Kettle,
            "mashtun" => ResourceType::MashTun,
            "lautertun" => ResourceType::LauterTun,
            "keg" => ResourceType::Keg,
            "kegerator" => ResourceType::Kegerator,
            _ => ResourceType::Other(res.to_string())
        }
    }
}

impl From<String> for ResourceType {
    /// Convert from a `String` to a `ResourceType`.
    ///
    /// # Arguments
    /// * `res`: A `String` to be converted.
    ///
    /// # Returns
    ///
    /// * A `ResourceType` corresponding to the appropriate `&str` reference.
    ///
    /// # Notes
    ///
    /// This is a convenience function that converts the `String` to a string slice (`&str`) using
    /// `&res[..]` and calls the other variant of `From::<&str>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chronogrog::resources::ResourceType;
    /// let s = "fermentor".to_string();
    ///
    /// assert_eq!(ResourceType::Fermentor, ResourceType::from(s));
    /// ```
    fn from(res: String) -> Self {
        ResourceType::from(&res[..])
    }
}

/// Implementation of `serde_json::de::Deserializer` for `ResourceType` instances.
impl<'de> Deserialize<'de> for ResourceType {
    /// Deserialize from JSON into a `ResourceType`.
    ///
    /// # Arguments
    /// * `deserializer` - A `serde::de::Deserializer` that will be used for deserialization.
    ///
    /// # Returns
    /// * A `Result` containing a `ResourceType` as deserialized from JSON, or an `Error` if the
    ///   deserialization failed.
    ///
    /// # Errors
    /// * An `Error`, if the deserialization failed (most commonly associated with malformed JSON).
    ///   The string component of the `Error` will explain why the deserialization failed, and at
    ///   what location in the JSON the failure occurred.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let s = String::deserialize(deserializer)?;
            Ok(ResourceType::from(s.as_str()))
    }
}

/// Serialize to JSON from a `ResourceType`.
///
/// # Arguments
/// * `serializer` - A `serde::ser::Serializer` that will be used for serialization.
///
/// # Returns
/// * A `Result` containing a `String` containing serialized JSON, or an `Error` if the
///   serialization failed.
///
/// # Errors
/// * An `Error`, if the serialization failed.

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

/// A piece of equipment that must be used in order to produce a `Recipe`.
pub struct Resource {
    pub id: usize,
    pub name: String,

    #[serde(rename="type")]
    pub resource_type: ResourceType,

    #[serde(rename="capacity")]
    pub capacity_str: String
}
