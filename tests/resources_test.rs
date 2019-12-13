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
