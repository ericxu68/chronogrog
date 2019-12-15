extern crate chrono;
use chrono::Duration;
use chrono::NaiveDateTime;

use chronogrog::resources::Resource;
use chronogrog::resources::ResourceTracker;
use chronogrog::resources::ResourceType;

extern crate serde_test;
use serde_test::{Token, assert_tokens};

#[test]
fn it_should_convert_strings_and_slices_to_a_resourcetype() {
    let s: String = "fermentor".to_string();
    let s2 = s.clone();

    assert_eq!(ResourceType::Fermentor, ResourceType::from(s));

    assert_eq!(ResourceType::Fermentor, ResourceType::from(&s2[..]));
}

#[test]
fn test_ser_de_resourcetype() {
    let kettle = ResourceType::Kettle;

    assert_tokens(&kettle, &[
        Token::Str("kettle")
    ]);

    let mashtun = ResourceType::MashTun;

    assert_tokens(&mashtun, &[
        Token::Str("mashtun")
    ]);

    let lautertun = ResourceType::LauterTun;

    assert_tokens(&lautertun, &[
        Token::Str("lautertun")
    ]);

    let keg = ResourceType::Keg;

    assert_tokens(&keg, &[
        Token::Str("keg")
    ]);

    let kegerator = ResourceType::Kegerator;

    assert_tokens(&kegerator, &[
        Token::Str("kegerator")
    ]);

    let other = ResourceType::Other("fancythingy".to_string());

    assert_tokens(&other, &[
        Token::Str("fancythingy")
    ]);
}

#[test]
fn it_should_track_a_resource() {
    let mut tracker: ResourceTracker = ResourceTracker::new();
    let resource1: Resource = Resource {
        id: 2008,
        resource_type: ResourceType::Kettle,
        capacity_str: "15g".to_string(),
        name: "Large Kettle".to_string()
    };

    let resource1_copy = resource1.clone();

    tracker.track_resource(resource1);

    let current_date: NaiveDateTime = NaiveDateTime::parse_from_str("2019-01-01 00:00:00",
                                                                    "%Y-%m-%d %H:%M:%S").unwrap();

    let allocated_resource : &Resource =
      tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle, current_date,
                                                     Duration::days(10)).unwrap();
    assert_eq!(&resource1_copy, allocated_resource);
    assert_eq!(current_date.checked_add_signed(Duration::days(10)),
               tracker.next_available_resource_date_for_type(current_date, ResourceType::Kettle));

    let allocated_resource2 : Option<&Resource> =
        tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle, current_date,
                                                       Duration::days(10));
    assert_eq!(None, allocated_resource2);
}

#[test]
fn it_should_return_the_earliest_free_date_for_a_resource_of_a_type() {
    let mut tracker: ResourceTracker = ResourceTracker::new();
    let resource1: Resource = Resource {
        id: 2008,
        resource_type: ResourceType::Kettle,
        capacity_str: "15g".to_string(),
        name: "Large Kettle".to_string()
    };

    let resource2: Resource = Resource {
        id: 2009,
        resource_type: ResourceType::Kettle,
        capacity_str: "15g".to_string(),
        name: "Large Kettle No. 2".to_string()
    };

    tracker.track_resource(resource1);
    tracker.track_resource(resource2);

    let current_date: NaiveDateTime = NaiveDateTime::parse_from_str("2019-01-01 00:00:00",
                                                                    "%Y-%m-%d %H:%M:%S").unwrap();

    tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle, current_date,
                                                   Duration::days(10)).unwrap();

    tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle, current_date,
                                                   Duration::days(20)).unwrap();

    assert_eq!(current_date.checked_add_signed(Duration::days(10)),
               tracker.next_available_resource_date_for_type(current_date, ResourceType::Kettle));
}

#[test]
fn it_should_return_none_for_the_earliest_free_date_when_no_resources_are_allocated() {
    let mut tracker: ResourceTracker = ResourceTracker::new();
    let resource1: Resource = Resource {
        id: 2008,
        resource_type: ResourceType::Kettle,
        capacity_str: "15g".to_string(),
        name: "Large Kettle".to_string()
    };

    tracker.track_resource(resource1);

    let current_date: NaiveDateTime = NaiveDateTime::parse_from_str("2019-01-01 00:00:00",
                                                                    "%Y-%m-%d %H:%M:%S").unwrap();

    assert_eq!(None, tracker.next_available_resource_date_for_type(current_date,
                                                                   ResourceType::Fermentor));
}

#[test]
fn it_should_return_a_resource_as_free_after_the_free_date() {
    let mut tracker: ResourceTracker = ResourceTracker::new();
    let resource1: Resource = Resource {
        id: 2008,
        resource_type: ResourceType::Kettle,
        capacity_str: "15g".to_string(),
        name: "Large Kettle".to_string()
    };

    let resource1_copy = resource1.clone();
    tracker.track_resource(resource1);

    let mut current_date: NaiveDateTime = NaiveDateTime::parse_from_str("2019-01-01 00:00:00",
                                                                        "%Y-%m-%d %H:%M:%S")
                                                                        .unwrap();

    let mut allocated = tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle,
                                                                       current_date,
                                                                       Duration::days(10)).unwrap();
    assert_eq!(resource1_copy.id, allocated.id);

    current_date = NaiveDateTime::parse_from_str("2019-01-12 00:00:00",
                                                 "%Y-%m-%d %H:%M:%S").unwrap();


    allocated = tracker.allocate_resource_of_type_for_duration(ResourceType::Kettle,
                                                               current_date,
                                                               Duration::days(10)).unwrap();
    assert_eq!(resource1_copy.id, allocated.id);
}
