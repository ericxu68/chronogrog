use chrono::Duration;
use chrono::NaiveDateTime;

use chrono_period::NaivePeriod;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

use std::collections::HashMap;

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

    /// A tank for force-carbonating beer.
    GasTank,

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
            "gastank" => ResourceType::GasTank,
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
            ResourceType::GasTank => "gastank",
            ResourceType::Other(ref other) => other
        })
    }
}

/// A piece of equipment that must be used in order to produce a `Recipe`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Resource {
    pub id: usize,
    pub name: String,

    #[serde(rename="type")]
    pub resource_type: ResourceType,

    #[serde(rename="capacity")]
    pub capacity_str: String,

    #[serde(skip_serializing, skip_deserializing, default="Vec::new")]
    pub allocated_periods: Vec<NaivePeriod>
}

impl Resource {
    pub fn new(id: usize, name: &str, resource_type: ResourceType,
               capacity_str: &str) -> Resource {
        Resource {
            id: id,
            name: name.to_string(),
            resource_type: resource_type,
            capacity_str: capacity_str.to_string(),
            allocated_periods: vec![]
        }
    }

    /// Determine if this `Resource` is allocated at any time during a specific `Duration` starting
    // at a specific `NaiveDateTime`.
    pub fn is_allocated_over_start_duration(&self, start: NaiveDateTime,
                                            duration: Duration) -> bool {
        let intersection_period = NaivePeriod::from_start_duration(start, duration);

        self.allocated_periods.iter().any(|period| {
            period.intersects_with(intersection_period)
        })
    }

    pub fn is_allocated_over_period(&self, period: NaivePeriod) -> bool {
        self.is_allocated_over_start_duration(period.start, period.duration())
    }

    pub fn allocate_over_start_duration(&mut self, start: NaiveDateTime,
                                        duration: Duration) -> Option<&Resource> {
        if self.is_allocated_over_start_duration(start, duration) {
            return None;
        }

        let allocation_period = NaivePeriod::from_start_duration(start, duration);

        self.allocated_periods.push(allocation_period);
        self.allocated_periods.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        Some(self)
    }

    pub fn allocate_over_period(&mut self, period: NaivePeriod) -> Option<&Resource> {
        self.allocate_over_start_duration(period.start, period.duration())
    }

    pub fn get_earliest_free_date_for_period(&self, period: NaivePeriod) -> NaiveDateTime {
        if !self.is_allocated_over_start_duration(period.start, period.duration()) {
            return period.start;
        }

        self.allocated_periods.iter().filter(|needle| {
            // Discard any where the end date is before the desired start date.
            if needle.end.timestamp() < period.start.timestamp() {
                return false;
            }

            let end_date_plus_one_second = needle.end + Duration::seconds(1);
            let is_alloc = !self.is_allocated_over_start_duration(end_date_plus_one_second, period.duration());
            is_alloc
        }).map(|needle| {
            needle.end + Duration::seconds(1)
        }).take(1).next().unwrap()
    }
}

#[derive(Clone)]
/// A `Resource` that may be allocated (and thus not usable).
///
/// The majority of the time, the `Resource` bound to this object will be in use, and thus is not
/// available for use. However, at the border of expiration, the object may not yet have been freed
/// within the `ResourceTracker` (hence, the name 'PossiblyAllocated').
pub struct PossiblyAllocatedResource {
    /// A borrowed reference to the `Resource` that is currently in use.
    pub resource: Resource,

    /// The `NaiveDateTime` upon which the `Resource` will be free again. Given the current
    /// `NaiveDateTime`, `c`, if `free_date` is on or before `c`, the `Resource` should be freed.
    pub free_date: NaiveDateTime
}

/// Tracking mechanism for `Resource`s.
///
/// All usage of `Resource` objects shoiuld be tracked through this data structure. This provides
/// an API for retrieving an unused `Resource` of a given type.
///
/// This data structure tracks used and unused `Resource` objects. For `Resource`s that are
/// currently in use (and thus cannot be allocated to a new task), each object is tracked using the
/// `AllocatedResource` structure.
///
/// For unused `Resources`, a borrowed reference to the individual `Resource` is available.
#[derive(Debug)]
pub struct ResourceTracker {
    resources: HashMap<usize, Resource>
}

impl ResourceTracker {
    /// Create a new `ResourceTracker` to track [Resource](Resource)
    /// objects.
    ///
    /// `Resource` objects are tracked as _in use_ or _free_. Free resources can be allocated so
    /// that the date at which they are once again free can be tracked. This is necessary for
    /// scheduling resources. `Resource`s that are in use cannot be scheduled a second time, but
    /// they can be queried for when they will be free.
    ///
    /// # Notes
    ///
    /// - The `ResourceTracker` created is initially empty. It is necessary to add `Resource`s
    ///   manually using the [track_resource](ResourceTracker::track_resource) method.
    /// - The `ResourceTracker` tracks `Resource` objects by `id`. If a `Resource` object's `id` is
    ///   the same as a `Resource` already being tracked, the previously tracked `Resource` will be
    ///   dropped from the tracker.
    ///
    pub fn new() -> Self {
        ResourceTracker {
            resources: HashMap::new()
        }
    }

    /// Track a `Resource` using this `ResourceTracker`.
    ///
    /// Calling this method moves the `Resource` in question to be owned by this `ResourceTracker`.
    /// Thus, the lifetime of the `Resource` is bound to the lifetime of this `ResourceTracker`,
    /// and only borrowed references should be used.
    ///
    /// # Arguments
    ///
    /// * `res`: A  `Resource` object to track within this `ResourceTracker`.
    ///
    pub fn track_resource(&mut self, res: Resource) {
        self.resources.insert(res.id, res);
    }

    pub fn is_resource_of_type_free_for_period(&self, resource_type: &ResourceType,
                                               period: NaivePeriod) -> bool {
      self.resources.iter()
        .filter(|res| res.1.resource_type == *resource_type)
        .any(|res| !res.1.is_allocated_over_period(period))
    }

    /// Retrieve the next [NaiveDateTime](chrono::NaiveDateTime) at which a `Resource` of a
    /// specific `ResourceType` will be free.
    ///
    /// # Arguments
    ///
    /// * `resource_type`: The [ResourceType](ResourceType) to query for.
    ///
    /// # Returns
    ///
    /// * An `Option` containing one of the following values:
    ///   * `Some`: Contains an instance of type [NaiveDateTime](chrono::NaiveDateTime) that
    ///     represents the closest date at which a `Resource` of type `resource_type` will be free,
    ///     if there is at least one `Resource` of type `resource_type` allocated.
    ///   * `None`: If there are no `Resource`s of type `resource_type` that can be allocated.
    ///
    pub fn get_next_available_resource_date_for_type_over_period(&mut self,
                                                                 resource_type: &ResourceType,
                                                                 period: NaivePeriod)
      -> Option<NaiveDateTime> {

      let mut free_dates: Vec<NaiveDateTime> = self.resources.iter()
        .filter(|res| res.1.resource_type == *resource_type)
        .map(|res| {
            res.1.get_earliest_free_date_for_period(period)
        }).collect();

        free_dates.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if free_dates.is_empty() {
            return None;
        }

        Some(free_dates[0])
    }

    pub fn allocate_resource_of_type_for_period(&mut self, resource_type: &ResourceType,
                                                period: NaivePeriod) -> Option<&Resource> {
      // println!("Requesting resource of type {:?} for period starting {:?}", resource_type,
      //          period.start);

      if !self.is_resource_of_type_free_for_period(resource_type, period) {
          return None
      }

      // Because of the above check, we know this will return an element, so the unwrap() should
      // never panic here.
      let mut resources: Vec<(&usize, &mut Resource)> = self.resources.iter_mut().collect::<Vec<(&usize, &mut Resource)> >();
      resources.sort_by(|a, b| a.0.cmp(b.0));

      let resource_tuple: (&usize, &mut Resource) = resources.into_iter()
          .filter(|hash_entry| {
            hash_entry.1.resource_type == *resource_type
              && !hash_entry.1.is_allocated_over_period(period)
          }).next().unwrap();

      let ret_val = resource_tuple.1.allocate_over_period(period);

      // println!("Allocated resource of type: {:?}", ret_val.unwrap().resource_type);

      ret_val
    }

    pub fn get_all_tracked_resources(&self) -> Vec<Resource> {
        self.resources.clone().into_iter().map(|tuple| tuple.1).collect()
    }
}
