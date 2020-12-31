use std::collections::HashMap;

pub type WatsonCharacter = u8;
pub type WChar = WatsonCharacter;
pub type WatsonString = Vec<WChar>;
pub type WString = WatsonString;

#[derive(Debug, Clone)]
pub enum Type {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(WString),
    Object(HashMap<WString, Type>),
    Array(Vec<Type>),
    Bool(bool),
    Nil,
}

impl Type {
    pub fn int_to_wchar(int: i64) -> WChar {
        (int & !0xff) as WChar
    }
}
