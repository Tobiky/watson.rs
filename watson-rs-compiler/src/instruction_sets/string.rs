use watson_rs_core::types::{Type, WString};

pub fn snew(stack: &mut Vec<Type>) {
    stack.push(Type::String(WString::new()))
}

pub fn sadd(stack: &mut Vec<Type>) {
    let token_x = stack.pop().unwrap();
    let token_s = stack.pop().unwrap();

    if let Type::Int(x) = token_x {
        if let Type::String(mut y) = token_s {
            y.push(Type::int_to_wchar(x));
            stack.push(Type::String(y));
            return;
        }
    }

    panic!("tried popping non-Int or non-String object from stack");
}
