use watson_rs_core::types::Type;

pub fn anew(stack: &mut Vec<Type>) {
    stack.push(Type::Array(Vec::new()))
}

pub fn aadd(stack: &mut Vec<Type>) {
    let token_x = stack.pop().unwrap();
    let token_a = stack.pop().unwrap();

    if let Type::Array(mut a) = token_a {
        return a.push(token_x);
    }

    panic!("tried popping non-Array object from stack");
}
