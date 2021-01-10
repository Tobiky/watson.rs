// TODO: ASCII/Byte feature; look at lexemes as bytes instead (reduces data size)
pub mod evaluator;
pub mod lexer;
pub mod scanner;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
