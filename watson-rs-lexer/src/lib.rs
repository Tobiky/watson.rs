pub mod error;
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
