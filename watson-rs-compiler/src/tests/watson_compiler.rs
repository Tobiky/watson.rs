use watson_rs_lexer::lexer::Lexer;

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
}

// https://github.com/genkami/watson#string
#[test]
fn string_example() {}

// https://github.com/genkami/watson#hello-world
#[test]
fn hello_world_example() {}
