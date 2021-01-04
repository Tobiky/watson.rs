use std::collections::HashMap;

use watson_rs_core::types::Type;

pub fn onew(stack: &mut Vec<Type>) {
    stack.push(Type::Object(HashMap::new()))
}

pub fn oadd(stack: &mut Vec<Type>) {
    let token_v = stack.pop().unwrap();
    let token_k = stack.pop().unwrap();
    let token_o = stack.pop().unwrap();

    if let Type::String(k) = token_k {
        if let Type::Object(mut o) = token_o {
            o.insert(k, token_v);
            return;
        }
    }

    panic!("tried popping non-String or non-Object object from stack");
}
