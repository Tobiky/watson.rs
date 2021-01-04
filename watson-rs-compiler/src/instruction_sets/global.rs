use watson_rs_core::types::Type;

pub fn gdup(stack: &mut Vec<Type>) {
    let token_x = stack.pop().unwrap();

    stack.push(token_x.clone());
    stack.push(token_x);
}

pub fn gpop(stack: &mut Vec<Type>) {
    stack.pop();
}

pub fn gswp(stack: &mut Vec<Type>) {
    let token_y = stack.pop().unwrap();
    let token_x = stack.pop().unwrap();

    stack.push(token_y);
    stack.push(token_x);
}
