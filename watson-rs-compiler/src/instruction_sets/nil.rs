use watson_rs_core::types::Type;

pub fn nnew(stack: &mut Vec<Type>) -> Result<(), &str>{
    Ok(stack.push(Type::Nil))
}
