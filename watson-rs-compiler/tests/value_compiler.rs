use std::collections::HashMap;

use watson_rs_core::types::{Type, WString};
use watson_rs_lexer::lexer::Lexer;

use watson_rs_compiler::{value_compiler::ValueCompiler, Compiler};

// https://github.com/genkami/watson#integer
#[test]
fn integer_example() {
    let mut lexer = Lexer::new();
    let watson = "BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba";
    let result = lexer.tokenize_str(watson);

    assert!(result.is_ok(), result.err().unwrap().display_message());

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();
    let stack = if result.is_ok() { 
        result.ok().unwrap() 
    } else { 
        panic!(result.err().unwrap().display_message());
    };

    assert_eq!(
        stack.len(),
        1,
        "expected length 1, found length {}",
        stack.len()
    );

    let type_object = stack[0].clone();

    assert!(
        matches!(type_object, Type::Int(_)),
        "expected int type found {:?}",
        type_object
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

    assert!(result.is_ok(), result.err().unwrap().display_message());

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();
    let stack = if result.is_ok() {
        result.ok().unwrap()
    } else {
        panic!(result.err().unwrap().display_message());
    };

    assert_eq!(
        stack.len(),
        1,
        "expected length 1, found length {}",
        stack.len()
    );

    let type_object = stack[0].clone();

    assert!(
        matches!(type_object, Type::String(_)),
        "expected string type found {:?}",
        type_object
    );

    let value = type_object.as_string();

    #[cfg(feature = "ascii")]
    assert_eq!(
        b"tako",
        value.as_slice(),
        "expected {:?} found {:?}",
        b"tako",
        value.as_slice()
    );

    #[cfg(not(feature = "ascii"))]
    assert_eq!(
        String::from("tako"),
        value,
        "expected {:?} found {:?}",
        String::from("tako"),
        value
    );
}

// https://github.com/genkami/watson#hello-world
#[test]
fn hello_world_example() {
    let mut lexer = Lexer::new();
    // FIXME: The 'g' in the very end is causing the error
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

    assert!(result.is_ok(), result.err().unwrap().display_message());

    let instructions = result.ok().unwrap();
    let value_compiler = ValueCompiler::new(instructions);
    let result = value_compiler.compile();
    let stack = if result.is_ok() {
        result.ok().unwrap()
    } else {
        panic!(result.err().unwrap().display_message());
    };

    assert_eq!(
        stack.len(),
        1,
        "expected length 1, found length {}",
        stack.len()
    );

    let type_object = stack[0].clone();

    assert!(
        matches!(type_object, Type::Object(_)),
        "expected object type found {:?}",
        type_object
    );

    let value = type_object.as_object();

    #[cfg(feature = "ascii")]
    let expected = [
        (b"first".to_vec(), Type::Bool(true)),
        (b"hello".to_vec(), Type::String(b"world".to_vec())),
    ]
    .iter()
    .map(|x| x.clone())
    .collect::<HashMap<WString, Type>>();

    #[cfg(not(feature = "ascii"))]
    let expected = [
        (String::from("first"), Type::Bool(true)),
        (String::from("hello"), Type::String(String::from("world"))),
    ]
    .iter()
    .map(|x| x.clone())
    .collect::<HashMap<WString, Type>>();

    assert!(
        value.iter().eq(expected.iter()),
        "expected {:?} found {:?}",
        expected,
        value
    )
}
