use watson_rs_core::types::{Type, WString};

pub fn snew(stack: &mut Vec<Type>) -> Result<(), &str>{
    Ok(stack.push(Type::String(WString::new())))
}

pub fn sadd(stack: &mut Vec<Type>) -> Result<(), &str>{
    let option_token_x = stack.pop();
    let option_token_s = stack.pop();

    if option_token_x.is_none() || option_token_s.is_none() {
        return Err("tried popping value from empty stack");
    }

    let token_x = option_token_x.unwrap();
    let token_s = option_token_s.unwrap();

    if let Type::Int(x) = token_x {
        if let Type::String(mut y) = token_s {
            y.push(Type::int_to_wchar(x));
            stack.push(Type::String(y));
            return Ok(());
        }
        return Err("tried popping non-String object from stack");
    }

    Err("tried popping non-Int object from stack")
}
