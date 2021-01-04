use watson_rs_core::types::Type;

pub fn bnew(stack: &mut Vec<Type>) {
    stack.push(Type::Bool(false))
}

pub fn bneg(stack: &mut Vec<Type>) {
    let token_x = stack.pop().unwrap();

    if let Type::Bool(x) = token_x {
        return stack.push(Type::Bool(!x));
    }

    panic!("tried popping non-Bool object from stack");
}
