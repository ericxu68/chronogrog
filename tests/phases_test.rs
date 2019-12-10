use chrono::Duration;

use chronogrog::phases::PhaseInstance;

#[test]
fn it_should_construct_a_new_phaseinstance() {
    let phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                            "#FFFFFF".to_string(), Duration::days(10));

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
                                                "#FFFFFF".to_string(), Duration::days(10));

    phase_instance.add_dependency(2);
    phase_instance.add_dependency(2);

    let deps: Vec<usize> = vec![2];
    assert_eq!(deps, phase_instance.dependencies);
}

#[test]
fn it_should_show_phaseinstance_has_two_dependencies() {
    let mut phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                                "#FFFFFF".to_string(), Duration::days(10));

    phase_instance.add_dependency(3);
    phase_instance.add_dependency(2);

    let deps: Vec<usize> = vec![2, 3];
    assert_eq!(deps, phase_instance.dependencies);
}

#[test]
fn it_should_output_standard_pla_for_a_phaseinstance_with_two_dependencies() {
    let mut phase_instance = PhaseInstance::new(0, "A phase instance".to_string(),
                                                "#FFFFFF".to_string(), Duration::days(10));

    phase_instance.add_dependency(3);
    phase_instance.add_dependency(2);

    assert_eq!("[0] A phase instance\n  color #FFFFFF\n  duration 240\n    dep 2\n    dep 3\n\n",
               phase_instance.get_string_in_pla_format(0));
}
