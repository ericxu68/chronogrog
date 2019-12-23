use chrono::Duration;

use chronogrog::phases::{PhaseInstance, ProductionPhaseTemplate};
use chronogrog::resources::ResourceType;
use chronogrog::util::get_naive_date_time_from_string;

#[test]
fn it_should_deserialize_a_production_phase_template_from_json() {
 let json = r#"{
       "description": "Available to Drink",
       "id": "ready",
       "order": 6,
       "defaultDuration": "6m",
       "resourcesNeeded": [ "keg" ]
   }"#;

   let result: ProductionPhaseTemplate = serde_json::from_str(json).unwrap();
   assert_eq!(Some(Duration::days(6*30)), result.default_duration());
   assert_eq!(vec!(ResourceType::Keg), result.resources_needed);
   assert_eq!("ready", result.id);
   assert_eq!(6, result.order);
   assert_eq!("Available to Drink", result.description);
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
fn it_should_construct_a_new_phaseinstance() {
    let phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                            "#FFFFFF".to_string(), Duration::days(10),
                                            get_naive_date_time_from_string("2020-01-01").unwrap());

    assert_eq!(0, phase_instance.id);
    assert_eq!("A phase instance".to_string(), phase_instance.description);
    assert_eq!("#FFFFFF".to_string(), phase_instance.color_hex);
    assert_eq!(Duration::days(10), phase_instance.duration);

    let deps: Vec<usize> = vec![];
    assert_eq!(deps, phase_instance.dependencies);
}

#[test]
fn it_should_show_phaseinstance_has_one_dependency() {
    let mut phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                                "#FFFFFF".to_string(), Duration::days(10),
                                                get_naive_date_time_from_string("2020-01-01").unwrap());

    phase_instance.add_dependency(2);
    phase_instance.add_dependency(2);

    let deps: Vec<usize> = vec![2];
    assert_eq!(deps, phase_instance.dependencies);
}

#[test]
fn it_should_show_phaseinstance_has_two_dependencies() {
    let mut phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                                "#FFFFFF".to_string(), Duration::days(10),
                                                get_naive_date_time_from_string("2020-01-01").unwrap());

    phase_instance.add_dependency(3);
    phase_instance.add_dependency(2);

    let deps: Vec<usize> = vec![2, 3];
    assert_eq!(deps, phase_instance.dependencies);
}

#[test]
fn it_should_output_standard_pla_for_a_phaseinstance_with_two_dependencies() {
    let mut phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                                "#FFFFFF".to_string(), Duration::days(10),
                                                get_naive_date_time_from_string("2020-01-01").unwrap());

    phase_instance.add_dependency(3);
    phase_instance.add_dependency(2);

    assert_eq!("[0] A phase instance\n  start 2020-01-01\n  color #FFFFFF\n  duration 240\n    dep 2\n    dep 3\n\n",
               phase_instance.get_string_in_pla_format(0));
}
