// TODO: rename module to instructions

use std::{
    collections::HashMap,
    f64::{INFINITY, NAN},
};

use watson_rs_core::types::{Type, WString};

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

pub fn snew(stack: &mut Vec<Type>) {
    stack.push(Type::String(WString::new()))
}

pub fn sadd(stack: &mut Vec<Type>) {
    let token_x = stack.pop().unwrap();
    let token_s = stack.pop().unwrap();

    if let Type::Int(x) = token_x {
        if let Type::String(mut y) = token_s {
            y.push(Type::int_to_wchar(x));
            return stack.push(Type::String(y));
        }
    }

    panic!("tried popping non-Int or non-String object from stack");
}

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

pub fn nnew(stack: &mut Vec<Type>) {
    stack.push(Type::Nil)
}

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
