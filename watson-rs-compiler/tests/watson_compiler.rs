use watson_rs_core::{types::Type};
use watson_rs_lexer::lexer::Lexer;
use watson_rs_compiler::{Compiler, watson_compiler::WatsonCompiler};

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

     let compiler_instructions = watson_compiler.compile();

     let mut lexer = Lexer::new();
     let string_instructions_result = lexer.tokenize_str("BBuaBubaBubbbaBubbbbaBubbbbbaBubbbbbba");

     assert!(
         string_instructions_result.is_ok(),
         string_instructions_result.err().unwrap().display_message()
     );

     let string_instructions = string_instructions_result.ok().unwrap();

     assert_eq!(
        string_instructions, compiler_instructions,
         "expected {:?} found {:?}", string_instructions, compiler_instructions
     );
}

// https://github.com/genkami/watson#string
#[test]
fn string_example() {

}

// https://github.com/genkami/watson#hello-world
#[test]
fn hello_world_example() {

}
