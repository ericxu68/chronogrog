// #[macro_use]
// extern crate assert_json_diff;

// #[macro_use]
// extern crate serde_json;

extern crate tint;
use tint::Color;

use chrono::{NaiveDate, Duration};

use chronogrog::ProductionSchedule;
use chronogrog::ProductionPhaseTemplate;
use chronogrog::ResourceType;

#[test]
fn it_should_load_a_json_file_into_a_new_production_schedule() {
    let mut ps = ProductionSchedule::new("tests/fixtures/productionSchedule.json");

    assert_eq!("Simple Production Schedule", ps.name);
    assert_eq!(1, ps.id);
    assert_eq!("calendar", ps.timeline.configuration);

    let expected_date = NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d");

    assert_eq!(expected_date, ps.timeline.start_date());

    assert_eq!("Planning", ps.phase_templates[0].description);

    assert_eq!(Some(Duration::hours(1)), ps.phase_templates[0].default_duration());

    assert_eq!("Primary Fermentation", ps.phase_templates[2].description);
    assert_eq!(Some(Duration::days(10)), ps.phase_templates[2].default_duration());

    assert_eq!("Secondary Fermentation", ps.phase_templates[3].description);
    assert_eq!(Some(Duration::weeks(4)), ps.phase_templates[3].default_duration());

    assert_eq!(Some(ps.phase_templates[3].clone()), ps.get_phase_by_id("secondary"));

    assert_eq!(6, ps.resources.len());

    // There should be a kettle in the resources
    let resources = &ps.resources;
    let mut found = false;
    for next in resources {
        match next.resource_type {
            ResourceType::Kettle => {
                found = true;
            }
            _ => { found = found; }
        };
    }

    assert!(found);

    // Resource with id 1 should exist and be of type 'fermentor'
    match &ps.get_resource_by_id(1) {
        Some(x) => {
            assert_eq!(ResourceType::Fermentor, x.resource_type);
        },
        None => { assert!(false) }
    }

    let damned_squirrel = ps.get_recipe_by_name("Damned Squirrel Mk. II").unwrap();
    assert_eq!(damned_squirrel.name, "Damned Squirrel Mk. II");
    assert_eq!(Color::from_rgb255(122, 86, 36), damned_squirrel.color);

    // let damned_squirrel2 = ps.get_recipe_by_id(damned_squirrel.id).unwrap();
    // assert_eq!(damned_squirrel, damned_squirrel2);
}

#[test]
fn it_should_not_allow_for_an_empty_default_duration() {
    let funny_prod_schedule_json = r#"
        {
            "id": "erroneous",
            "description": "Erroneous Phase",
            "order": 39182,
            "defaultDuration": ""
        }
    "#;

    let result: ProductionPhaseTemplate = serde_json::from_str(funny_prod_schedule_json).unwrap();

    assert_eq!(None, result.default_duration());
}

#[test]
fn it_should_reject_an_unknown_specifier_for_default_duration() {
    let funny_prod_schedule_json = r#"
        {
            "id": "erroneous",
            "description": "Erroneous Phase",
            "order": 39182,
            "defaultDuration": "25x"
        }
    "#;

    let result: ProductionPhaseTemplate = serde_json::from_str(funny_prod_schedule_json).unwrap();

    assert_eq!(None, result.default_duration());
}

#[test]
#[should_panic]
fn it_should_panic_on_an_unparseable_json_file() {
    ProductionSchedule::new("tests/fixtures/bad_production_schedule.json");
}
