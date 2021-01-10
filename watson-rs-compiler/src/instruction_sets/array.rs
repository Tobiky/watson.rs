use watson_rs_core::types::Type;

pub fn anew(stack: &mut Vec<Type>) -> Result<(), &str> {
    Ok(stack.push(Type::Array(Vec::new())))
}

pub fn aadd(stack: &mut Vec<Type>) -> Result<(), &str> {
    let option_token_x = stack.pop();
    let option_token_a = stack.pop();

    if option_token_x.is_none() || option_token_a.is_none() {
        return Err("tried popping value from empty stack");
    }

    let token_x = option_token_x.unwrap();
    let token_a = option_token_a.unwrap();

    if let Type::Array(mut a) = token_a {
        return Ok(a.push(token_x));
    }

    Err("tried popping non-Array object from stack")
}
