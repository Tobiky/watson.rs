use watson_rs_core::{instructions::Instruction, types::Type};

use crate::{functions::create_type, Compiler};

pub struct WatsonCompiler {
    stack: Vec<Type>,
}

impl WatsonCompiler {
    pub fn new(stack: Vec<Type>) -> WatsonCompiler {
        WatsonCompiler { stack }
    }
}

impl Compiler for WatsonCompiler {
    type Out = Vec<Instruction>;

    fn compile(self) -> Self::Out {
        self.stack
            .iter()
            .fold(Vec::new(), |mut instructions, type_object| {
                create_type(&mut instructions, type_object);
                instructions
            })
    }
}
