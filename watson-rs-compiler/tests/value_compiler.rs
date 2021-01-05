use std::collections::HashMap;

use watson_rs_core::types::Type;
use watson_rs_lexer::lexer::Lexer;

use watson_rs_compiler::{value_compiler::ValueCompiler, Compiler};

// https://github.com/genkami/watson#integer
#[test]
fn integer_example() {
    let mut lexer = Lexer::new();
    let watson = "BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba";
    let result = lexer.tokenize_str(watson);

    assert!(
        result.is_ok(),
        "{}",
        result.err().unwrap().display_message()
    );

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();

    assert_eq!(
        result.len(), 1,
        "expected length 1, found length {}", result.len()
    );

    let type_object = result[0].clone();

    assert!(
        matches!(type_object, Type::Int(_)),
        "expected int type found {:?}", type_object
    );

    let value = type_object.as_int();

    assert_eq!(value, 123, "expected 123, found {}", value);
}

// https://github.com/genkami/watson#string
#[test]
fn string_example() {
    let mut lexer = Lexer::new();
    let watson = "?SShaakShaaaakShaaaaakShaaaaaak-SShkShaaaaakShaaaaaak-SShkShakShaaakShaaaaakShaaaaaak-SShkShakShaakShaaakShaaaaakShaaaaaak-";
    let result = lexer.tokenize_str(watson);

    assert!(
        result.is_ok(),
        "{}", result.err().unwrap().display_message()
    );

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();

    assert_eq!(
        result.len(), 1,
        "expected length 1, found length {}", result.len()
    );

    let type_object = result[0].clone();

    assert!(
        matches!(type_object, Type::String(_)),
        "expected string type found {:?}", type_object
    );

    let value = type_object.as_string();

    assert_eq!(
        b"tako",  value.as_slice(),
        "expected {:?} found {:?}", b"tako", value.as_slice()
    )
}

// https://github.com/genkami/watson#hello-world
#[test]
fn hello_world_example() {
    let mut lexer = Lexer::new();
    let watson = "~?ShaaaaaarrShaaaaarrkShaaarrk-
                        SameeShaaaaaarrShaaaaarrkShaarrkShrrk-
                        ShaaaaaarrShaaaaakSameeShaaarrkShaarrk-
                        ShaaaaaarrShaaaaarrkShaaarrkShaarrk-
                        ShaaaaaarrShaaaaarrkShaaarrkShaarrkSharrkShrrk-$
                        BubbbbbbBubbbbbaBubbbbaBubbaBubaBua!
                        BubbbbbbBubbbbbaBubbbaBubbaBubaBua!
                        BubbbbbbBubbbbbaBubbbbaBuba!
                        BubbbbbbBubbbbbaBubbbaBubba!
                        BubbbbbbBubbbbbaBubba!M?
                        ShaaaaaaShaaaaakShaakShak-
                        ShaaaaaaShaaaaakShaaakShk-
                        ShaaaaaaShaaaaakShaaaakShak-
                        ShaaaaaaShaaaaakShaaaakShakShk-
                        ShaaaaaaShaaaaakShaaaakShaak-
                        ^!!!!!!!!!!!!!g";
    let result = lexer.tokenize_str(watson);

    assert!(
        result.is_ok(),
        "{}", result.err().unwrap().display_message()
    );

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();

    assert_eq!(
        result.len(), 1,
        "expected length 1, found length {}", result.len()
    );

    let type_object = result[0].clone();

    assert!(
        matches!(type_object, Type::Object(_)),
        "expected object type found {:?}", type_object
    );

    let value = type_object.as_object();
    let expected = [(b"first".to_vec(), Type::Bool(true)), (b"hello".to_vec(), Type::String(b"world".to_vec()))]
        .iter()
        .map(|x| x.clone())
        .collect::<HashMap<Vec<u8>, Type>>();
    
    assert!(
        value.iter().eq(expected.iter()), 
        "expected {:?} found {:?}", expected, value
    )
}
