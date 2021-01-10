use std::collections::HashMap;

use watson_rs_core::types::Type;

pub fn onew(stack: &mut Vec<Type>) -> Result<(), &str>{
    Ok(stack.push(Type::Object(HashMap::new())))
}

pub fn oadd(stack: &mut Vec<Type>) -> Result<(), &str> {
    let option_token_v = stack.pop();
    let option_token_k = stack.pop();
    let option_token_o = stack.pop();

    if option_token_v.is_none() || option_token_o.is_none() || option_token_k.is_none() {
        return Err("tried popping value from empty stack");
    }

    let token_v = option_token_v.unwrap();
    let token_k = option_token_k.unwrap();
    let token_o = option_token_o.unwrap();

    if let Type::String(k) = token_k {
        if let Type::Object(mut o) = token_o {
            o.insert(k, token_v);
            return Ok(());
        }
        return Err("tried popping non-Object object from stack");
    }

    Err("tried popping non-String object from stack")
}
