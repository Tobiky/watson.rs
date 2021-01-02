use std::f64::{INFINITY, NAN};

    use watson_rs_core::types::Type;

    pub fn finf(stack: &mut Vec<Type>) {
        stack.push(Type::Float(INFINITY))
    }

    pub fn fnan(stack: &mut Vec<Type>) {
        stack.push(Type::Float(NAN))
    }

    pub fn fneg(stack: &mut Vec<Type>) {
        let token = stack.pop().unwrap();

        if let Type::Float(x) = token {
            return stack.push(Type::Float(-x));
        }

        panic!("tried popping non-Float object from stack");
    }
