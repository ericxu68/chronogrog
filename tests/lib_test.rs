// #[macro_use]
// extern crate assert_json_diff;

// #[macro_use]
// extern crate serde_json;

use chrono::{NaiveDate, Duration};

use chronogrog::ProductionSchedule;

#[test]
fn it_should_load_a_json_file_into_a_new_production_schedule() {
    let ps = ProductionSchedule::new("tests/fixtures/productionSchedule.json");

    assert_eq!("Simple Production Schedule", ps.name);
    assert_eq!(1, ps.id);
    assert_eq!("calendar", ps.timeline.configuration);

    let expected_date = NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d");

    assert_eq!(expected_date, ps.timeline.start_date());

    assert_eq!("Planning", ps.phases[0].description);

    assert_eq!(Some(Duration::hours(1)), ps.phases[0].default_duration());
}
