use std::f64::{INFINITY, NAN};

use watson_rs_core::types::{TokenType, WString, WatsonString};

pub fn inew(stack: &mut Vec<TokenType>)
{
    stack.push(TokenType::Int(0))
}
pub fn iinc(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Int(x) = token {
        stack.push(TokenType::Int(x + 1))
    }

    panic!("tried popping non-Int object from stack");
}
pub fn ishl(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Int(x) = token {
        stack.push(TokenType::Int(x << 1))
    }

    panic!("tried popping non-Int object from stack");
}
pub fn iadd(stack: &mut Vec<TokenType>)
{
    let token_y = stack.pop().unwrap();
    let token_x = stack.pop().unwrap();
    
    if let TokenType::Int(x) = token_x {
        if let TokenType::Int(y) = token_y {
            stack.push(TokenType::Int(x + y))
        }
    }

    panic!("tried popping non-Int object from stack");
}
pub fn ineg(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Int(x) = token {
        stack.push(TokenType::Int(-x))
    }

    panic!("tried popping non-Int object from stack");
}
pub fn isht(stack: &mut Vec<TokenType>)
{
    let token_y = stack.pop().unwrap();
    let token_x = stack.pop().unwrap();
    
    if let TokenType::Int(x) = token_x {
        if let TokenType::Int(y) = token_y {
            stack.push(TokenType::Int(x << y))
        }
    }

    panic!("tried popping non-Int object from stack");
}
pub fn itof(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Int(x) = token {
        stack.push(TokenType::Float(x as f64))
    }

    panic!("tried popping non-Int object from stack");
}
pub fn itou(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Int(x) = token {
        stack.push(TokenType::Uint(x as u64))
    }

    panic!("tried popping non-Int object from stack");
}
pub fn finf(stack: &mut Vec<TokenType>)
{
    stack.push(TokenType::Float(INFINITY))
}
pub fn fnan(stack: &mut Vec<TokenType>)
{
    stack.push(TokenType::Float(NAN))
}
pub fn fneg(stack: &mut Vec<TokenType>)
{
    let token = stack.pop().unwrap();

    if let TokenType::Float(x) = token {
        stack.push(TokenType::Float(-x))
    }

    panic!("tried popping non-Float object from stack");
}
pub fn snew(stack: &mut Vec<TokenType>)
{
    stack.push(TokenType::String(WatsonString::new()))
}
pub fn sadd(stack: &mut Vec<TokenType>)
{
    let token_x = stack.pop().unwrap();
    let token_s = stack.pop().unwrap();

    if let TokenType::Int(x) = token_x {
        if let TokenType::String(mut y) = token_s {
            y.push(TokenType::int_to_wchar(x));
            stack.push(TokenType::String(y))
        }
    }

    panic!("tried popping non-Int or non-String object from stack");
}
pub fn onew(stack: &mut Vec<TokenType>)
{
    stack.push(TokenType::Object((WString::new(), Box::new(TokenType::Nil))))
}
pub fn oadd(stack: &mut Vec<TokenType>)
{

}
pub fn anew(stack: &mut Vec<TokenType>)
{

}
pub fn aadd(stack: &mut Vec<TokenType>)
{

}
pub fn bnew(stack: &mut Vec<TokenType>)
{

}
pub fn bneg(stack: &mut Vec<TokenType>)
{

}
pub fn nnew(stack: &mut Vec<TokenType>)
{

}
pub fn gdup(stack: &mut Vec<TokenType>)
{

}
pub fn gpop(stack: &mut Vec<TokenType>)
{

}
pub fn gswp(stack: &mut Vec<TokenType>)
{

}