use watson_rs_core::{lexeme, types::Type};
use watson_rs_lexer::lexer::Lexer;
use watson_rs_compiler::watson_compiler::WatsonCompiler;

// https://github.com/genkami/watson#integer
#[test]
fn integer_example() {
     let type_object = Type::Int(123);
     let stack= vec![ type_object ];
     let watson_compiler = WatsonCompiler::new(stack);
     let string_result = watson_compiler.clone().string_compile();
     
     assert_eq!(
         string_result, "BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba",
         "expected `BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba` found `{}`", string_result
     );

     let mut lexer = Lexer::new();
     let instructions_result = lexer.tokenize_str("BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba");


}

// https://github.com/genkami/watson#string
#[test]
fn string_example() {

}

// https://github.com/genkami/watson#hello-world
#[test]
fn hello_world_example() {

}
