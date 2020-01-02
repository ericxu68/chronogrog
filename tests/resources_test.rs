extern crate chrono;
use chrono::Duration;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

extern crate chrono_period;
use chrono_period::NaivePeriod;

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
fn test_resource_allocation() {
    let mut resource: Resource = Resource::new(2008, "Large Kettle", ResourceType::Kettle, "15g");

    let allocated = resource
      .allocate_over_start_duration(NaiveDateTime::new(NaiveDate::from_ymd(2019, 12, 1),
                                                       NaiveTime::from_hms(0, 0, 0)),
                                    Duration::days(10));
    assert!(allocated.is_some());

    assert!(allocated.unwrap()
      .is_allocated_over_start_duration(NaiveDateTime::new(NaiveDate::from_ymd(2019, 12, 4),
                                                           NaiveTime::from_hms(12, 0, 26)),
                                        Duration::days(1)));
    assert!(!allocated.unwrap()
      .is_allocated_over_start_duration(NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 4),
                                                           NaiveTime::from_hms(0, 0, 0)),
                                        Duration::days(14)));

    let allocated_fail = resource
      .allocate_over_start_duration(NaiveDateTime::new(NaiveDate::from_ymd(2019, 12, 2),
                                                       NaiveTime::from_hms(0, 0, 0)),
                                    Duration::days(22));
    assert!(allocated_fail.is_none());
}

#[test]
fn test_get_resource_earliest_free_date() {
    let mut resource: Resource = Resource::new(2008, "Large Kettle", ResourceType::Kettle, "15g");
    let target_date = NaiveDateTime::parse_from_str("2020-01-01 00:00:00",
                                                    "%Y-%m-%d %H:%M:%S").unwrap();
    let target_period = NaivePeriod::from_start_duration(target_date, Duration::days(10));

    assert_eq!(target_date, resource.get_earliest_free_date_for_period(target_period));

    resource.allocate_over_start_duration(target_period.start, target_period.duration());

    let next_target_date = NaiveDateTime::parse_from_str("2020-01-04 00:00:00",
                                                         "%Y-%m-%d %H:%M:%S").unwrap();
    let next_target_period = NaivePeriod::from_start_duration(next_target_date, Duration::days(4));

    let good_start_date = NaiveDateTime::parse_from_str("2020-01-11 00:00:01",
                                                        "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(good_start_date, resource.get_earliest_free_date_for_period(next_target_period));
}

#[test]
fn test_get_next_available_resource_date_for_type_over_period_with_no_resources() {
    let mut tracker = ResourceTracker::new();
    let next_target_date = NaiveDateTime::parse_from_str("2020-01-04 00:00:00",
                                                         "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(next_target_date, Duration::days(4));
    assert!(tracker.get_next_available_resource_date_for_type_over_period(&ResourceType::LauterTun,
                                                                          period).is_none());
}

#[test]
fn test_next_available_resource_date_for_type_with_no_allocated_resources() {
    let mut tracker = ResourceTracker::new();
    let resource: Resource = Resource::new(2008, "Large Kettle", ResourceType::Kettle, "15g");

    tracker.track_resource(resource);

    let next_target_date = NaiveDateTime::parse_from_str("2020-01-04 00:00:00",
                                                         "%Y-%m-%d %H:%M:%S").unwrap();

    let period = NaivePeriod::from_start_duration(next_target_date, Duration::days(4));

    assert_eq!(next_target_date,
               tracker.get_next_available_resource_date_for_type_over_period(&ResourceType::Kettle,
                                                                             period).unwrap());
}

#[test]
fn test_two_resources_allocated_no_space_between() {
    // Resource 1 Allocation |----------|    |----------|
    // Resource 2 Allocation      |----------|
    // Resource Request             |-----|
    // Resource Available                    |-----|
    let mut resource1 = Resource::new(2008, "FV 001", ResourceType::Fermentor, "5g");
    let r1_allocation_start1 = NaiveDateTime::parse_from_str("2020-01-01 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let r1_allocation_start2 = NaiveDateTime::parse_from_str("2020-01-14 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let allocation1 = resource1.allocate_over_start_duration(r1_allocation_start1, Duration::days(10));
    assert!(allocation1.is_some());

    let allocation2 = resource1.allocate_over_start_duration(r1_allocation_start2, Duration::days(10));
    assert!(allocation2.is_some());

    let mut resource2 = Resource::new(2009, "FV 002", ResourceType::Fermentor, "5g");
    let r2_allocation_start1 = NaiveDateTime::parse_from_str("2020-01-05 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let allocation3 = resource2.allocate_over_start_duration(r2_allocation_start1, Duration::days(10));
    assert!(allocation3.is_some());

    let mut tracker = ResourceTracker::new();
    tracker.track_resource(resource1);
    tracker.track_resource(resource2);

    let desired_start = NaiveDateTime::parse_from_str("2020-01-07 00:00:00",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(desired_start, Duration::days(5));
    let avab_date =
      tracker.get_next_available_resource_date_for_type_over_period(&ResourceType::Fermentor,
                                                                    period);
    let expected_avab = NaiveDateTime::parse_from_str("2020-01-15 00:00:01",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(expected_avab, avab_date.unwrap());
}

#[test]
fn test_is_resource_of_type_free_over_period() {
    let mut resource1 = Resource::new(2008, "FV 001", ResourceType::Fermentor, "5g");
    let r1_allocation_start1 = NaiveDateTime::parse_from_str("2020-01-01 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let r1_allocation_start2 = NaiveDateTime::parse_from_str("2020-01-14 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let allocation1 = resource1.allocate_over_start_duration(r1_allocation_start1, Duration::days(10));
    assert!(allocation1.is_some());

    let allocation2 = resource1.allocate_over_start_duration(r1_allocation_start2, Duration::days(10));
    assert!(allocation2.is_some());

    let mut resource2 = Resource::new(2009, "FV 002", ResourceType::Fermentor, "5g");
    let r2_allocation_start1 = NaiveDateTime::parse_from_str("2020-01-05 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let allocation3 = resource2.allocate_over_start_duration(r2_allocation_start1, Duration::days(10));
    assert!(allocation3.is_some());

    let mut tracker = ResourceTracker::new();
    tracker.track_resource(resource1);
    tracker.track_resource(resource2);

    let desired_start = NaiveDateTime::parse_from_str("2020-01-07 00:00:00",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(desired_start, Duration::days(5));

    assert!(!tracker.is_resource_of_type_free_for_period(&ResourceType::Fermentor, period));
    assert!(!tracker.is_resource_of_type_free_for_period(&ResourceType::Kettle, period));


    let expected_avab = NaiveDateTime::parse_from_str("2020-01-15 00:00:01",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let expected_period = NaivePeriod::from_start_duration(expected_avab, Duration::days(5));
    assert!(tracker.is_resource_of_type_free_for_period(&ResourceType::Fermentor,
                                                        expected_period));
}

#[test]
fn test_allocate_resource_of_type_over_period_with_no_resources() {
    let mut tracker = ResourceTracker::new();

    let desired_start = NaiveDateTime::parse_from_str("2020-01-07 00:00:00",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(desired_start, Duration::days(5));

    assert_eq!(None, tracker.allocate_resource_of_type_for_period(&ResourceType::Fermentor,
                                                                  period));
}

#[test]
fn test_allocate_resource_of_type_over_period_with_no_free_resources_for_period() {
    let mut resource1 = Resource::new(2008, "FV 001", ResourceType::Fermentor, "5g");
    let r1_allocation_start1 = NaiveDateTime::parse_from_str("2020-01-01 00:00:00",
                                                             "%Y-%m-%d %H:%M:%S").unwrap();
    let allocation1 = resource1.allocate_over_start_duration(r1_allocation_start1, Duration::days(10));
    assert!(allocation1.is_some());

    let mut tracker = ResourceTracker::new();
    tracker.track_resource(resource1);

    let desired_start = NaiveDateTime::parse_from_str("2020-01-07 00:00:00",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(desired_start, Duration::days(5));

    assert_eq!(None, tracker.allocate_resource_of_type_for_period(&ResourceType::Fermentor,
                                                                  period));
}

#[test]
fn test_allocate_resource_of_type_over_period_with_single_free_resource() {
    let resource1 = Resource::new(2008, "FV 001", ResourceType::Fermentor, "5g");

    let mut tracker = ResourceTracker::new();
    tracker.track_resource(resource1);

    let desired_start = NaiveDateTime::parse_from_str("2020-01-07 00:00:00",
                                                      "%Y-%m-%d %H:%M:%S").unwrap();
    let period = NaivePeriod::from_start_duration(desired_start, Duration::days(5));

    let allocated = tracker.allocate_resource_of_type_for_period(&ResourceType::Fermentor, period);
    assert!(allocated.is_some());

    assert!(!tracker.is_resource_of_type_free_for_period(&ResourceType::Fermentor, period));

    let free_period = NaivePeriod::from_start_duration(desired_start + Duration::days(20),
                                                       Duration::days(15));
    assert!(tracker.is_resource_of_type_free_for_period(&ResourceType::Fermentor, free_period));
}

#[test]
fn test_several_resources_get_free_of_type() {
    let resource1 = Resource::new(1, "FV-001", ResourceType::Fermentor, "5g");
    let resource2 = Resource::new(2, "FV-002", ResourceType::Fermentor, "5g");
    let resource3 = Resource::new(3, "Scott's CO2 Tank",
                                  ResourceType::Other("gastank".to_string()), "5p");

  let mut tracker = ResourceTracker::new();
  tracker.track_resource(resource1);
  tracker.track_resource(resource2);
  tracker.track_resource(resource3);

  let allocation_start = NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 12),
                                            NaiveTime::from_hms(0, 0, 0));
  let allocation_period = NaivePeriod::from_start_duration(allocation_start, Duration::days(10));

  let allocated_resource = tracker.allocate_resource_of_type_for_period(&ResourceType::Fermentor,
                                                                        allocation_period);

  assert!(allocated_resource.is_some());

  let res = allocated_resource.unwrap();

  assert_eq!(ResourceType::Fermentor, res.resource_type);
  assert_eq!(1, res.id);
}

#[test]
fn test_long_term_allocation() {
    let keg1 = Resource::new(6, "Keg 001", ResourceType::Keg, "5g");
    let keg2 = Resource::new(7, "Keg 002", ResourceType::Keg, "5g");

    let mut tracker = ResourceTracker::new();
    tracker.track_resource(keg1);
    tracker.track_resource(keg2);

    let carb_period1 = NaivePeriod::from_start_duration(NaiveDate::from_ymd(2020, 1, 31).and_hms(4, 0, 2), Duration::days(10));

    let allocated_keg1_pd1 = tracker.allocate_resource_of_type_for_period(&ResourceType::Keg, carb_period1);

    assert!(allocated_keg1_pd1.is_some());
    assert_eq!(6, allocated_keg1_pd1.unwrap().id);

    let carb_period2 = NaivePeriod::from_start_duration(NaiveDate::from_ymd(2020, 4, 8).and_hms(4, 0, 0), Duration::days(10));
    let allocated_keg1_pd2 = tracker.allocate_resource_of_type_for_period(&ResourceType::Keg, carb_period2);

    assert!(allocated_keg1_pd2.is_some());
    assert_eq!(6, allocated_keg1_pd2.unwrap().id);

    let available_period1 = NaivePeriod::from_start_duration(NaiveDate::from_ymd(2020, 4, 18).and_hms(4, 0, 0), Duration::days(30*6));
    let allocated_keg2_pd1 = tracker.allocate_resource_of_type_for_period(&ResourceType::Keg, available_period1);

    assert!(allocated_keg2_pd1.is_some());
    assert_eq!(7, allocated_keg2_pd1.unwrap().id);

    let available_period2 = NaivePeriod::from_start_duration(NaiveDate::from_ymd(2020, 4, 18).and_hms(4, 0, 1), Duration::days(30*6));
    let allocated_keg1_pd3 = tracker.allocate_resource_of_type_for_period(&ResourceType::Keg, available_period2);

    assert!(allocated_keg1_pd3.is_some());
    assert_eq!(6, allocated_keg1_pd3.unwrap().id);

    // Now, the earliest possible free date for a resource of type Keg after 2020-02-16 04 should
    // be 2020-10-15T04:00:00
    let desired_start = NaiveDate::from_ymd(2020, 2, 16).and_hms(4, 0, 1) + Duration::days(64);
    let desired_period = NaivePeriod::from_start_duration(desired_start, Duration::days(10));

    let first_available_date = tracker.get_next_available_resource_date_for_type_over_period(&ResourceType::Keg, desired_period);

    assert!(first_available_date.is_some());
    assert_eq!(NaiveDate::from_ymd(2020, 10, 15).and_hms(4, 0, 1), first_available_date.unwrap());
}
