use watson_rs_core::types::Type;

pub fn inew(stack: &mut Vec<Type>) {
    stack.push(Type::Int(0))
}

pub fn iinc(stack: &mut Vec<Type>) {
    let token = stack.pop().unwrap();

    if let Type::Int(x) = token {
        stack.push(Type::Int(x + 1))
    }

    panic!("tried popping non-Int object from stack");
}

pub fn ishl(stack: &mut Vec<Type>) {
    let token = stack.pop().unwrap();

    if let Type::Int(x) = token {
        stack.push(Type::Int(x << 1))
    }

    panic!("tried popping non-Int object from stack");
}

pub fn iadd(stack: &mut Vec<Type>) {
    let token_y = stack.pop().unwrap();
    let token_x = stack.pop().unwrap();

    if let Type::Int(x) = token_x {
        if let Type::Int(y) = token_y {
            stack.push(Type::Int(x + y))
        }
    }

    panic!("tried popping non-Int object from stack");
}

pub fn ineg(stack: &mut Vec<Type>) {
    let token = stack.pop().unwrap();

    if let Type::Int(x) = token {
        stack.push(Type::Int(-x))
    }

    panic!("tried popping non-Int object from stack");
}

pub fn isht(stack: &mut Vec<Type>) {
    let token_y = stack.pop().unwrap();
    let token_x = stack.pop().unwrap();

    if let Type::Int(x) = token_x {
        if let Type::Int(y) = token_y {
            stack.push(Type::Int(x << y))
        }
    }

    panic!("tried popping non-Int object from stack");
}

pub fn itof(stack: &mut Vec<Type>) {
    let token = stack.pop().unwrap();

    if let Type::Int(x) = token {
        return stack.push(Type::Float(x as f64));
    }

    panic!("tried popping non-Int object from stack");
}

pub fn itou(stack: &mut Vec<Type>) {
    let token = stack.pop().unwrap();

    if let Type::Int(x) = token {
        return stack.push(Type::Uint(x as u64));
    }

    panic!("tried popping non-Int object from stack");
}
