use watson_rs_core::types::Type;

pub fn nnew(stack: &mut Vec<Type>) {
    stack.push(Type::Nil)
}
