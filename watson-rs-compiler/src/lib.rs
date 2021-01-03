pub(crate) mod functions;
pub(crate) mod instruction_sets;
pub mod value_compiler;
pub mod watson_compiler;

mod tests;

// TODO: Make builder for compiler?

// Compiles types and values into WATSON instructions or vice versa
pub trait Compiler {
    type Out;

    fn compile(self) -> Self::Out;
}