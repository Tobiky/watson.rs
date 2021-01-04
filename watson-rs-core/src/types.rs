#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

// TODO: Remove or privatize short or long names
pub type WatsonCharacter = u8;
pub type WChar = WatsonCharacter;

pub type WatsonString = Vec<WChar>;
pub type WString = WatsonString;

pub type WatsonObject = HashMap<WString, Type>;
pub type WObject = WatsonObject;

pub type WatsonArray = Vec<Type>;
pub type WArray = WatsonArray;

// TODO: Change WString to actual string and convert between stages (turn into feature to respect ASCII/Byte feature of lexer)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(WString),
    Object(WObject),
    Array(WArray),
    Bool(bool),
    Nil,
}

impl Type {
    pub fn int_to_wchar(int: i64) -> WChar {
        (int & !0xff) as WChar
    }

    pub fn as_int(self) -> i64 {
        match self {
            Type::Int(x) => x,
            _ => panic!("tried calling `Type::as_int()` on a non-int value"),
        }
    }

    pub fn as_uint(self) -> u64 {
        match self {
            Type::Uint(x) => x,
            _ => panic!("tried calling `Type::as_uint()` on a non-uint value"),
        }
    }

    pub fn as_float(self) -> f64 {
        match self {
            Type::Float(x) => x,
            _ => panic!("tried calling `Type::as_float()` on a non-float value"),
        }
    }

    pub fn as_string(self) -> WString {
        match self {
            Type::String(x) => x,
            _ => panic!("tried calling `Type::as_string()` on a non-string value"),
        }
    }

    pub fn as_object(self) -> WObject {
        match self {
            Type::Object(x) => x,
            _ => panic!("tried calling `Type::as_object()` on a non-object value"),
        }
    }

    pub fn as_array(self) -> WArray {
        match self {
            Type::Array(x) => x,
            _ => panic!("tried calling `Type::as_array()` on a non-array value"),
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Type::Bool(x) => x,
            _ => panic!("tried calling `Type::as_bool()` on a non-bool value"),
        }
    }
}
