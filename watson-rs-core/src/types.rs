#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use derive_more::TryUnwrap;
use std::collections::HashMap;

// TODO: Remove or privatize short or long names
#[cfg(feature = "ascii")]
pub type WatsonCharacter = u8;
#[cfg(not(feature = "ascii"))]
pub type WatsonCharacter = char;

pub type WChar = WatsonCharacter;

#[cfg(feature = "ascii")]
pub type WatsonString = Vec<WChar>;
#[cfg(not(feature = "ascii"))]
pub type WatsonString = String;

pub type WString = WatsonString;

pub type WatsonObject = HashMap<WString, Type>;
pub type WObject = WatsonObject;

pub type WatsonArray = Vec<Type>;
pub type WArray = WatsonArray;

// TODO: Change WString to actual string and convert between stages (turn into feature to respect ASCII/Byte feature of lexer)
#[derive(TryUnwrap, Debug, Clone, PartialEq)]
#[cfg_attr(any(feature = "serde", test), derive(Serialize, Deserialize))]
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
    pub const fn int_to_wchar(int: i64) -> WChar {
        #[cfg(feature = "ascii")]
        {
            return (int & !0xff) as WChar;
        }
        #[cfg(not(feature = "ascii"))]
        {
            return int as u8 as char;
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn to_int() {
        let int = Type::Int(-5);
        assert_eq!(Ok(-5), int.try_unwrap_int());
    }

    #[test]
    fn to_uint() {
        let uint = Type::Uint(6);
        assert_eq!(Ok(6), uint.try_unwrap_uint());
    }

    #[test]
    fn to_float() {
        let float = Type::Float(7.7);
        assert_eq!(Ok(7.7), float.try_unwrap_float());
    }

    #[test]
    fn to_string() {
        #[cfg(feature = "ascii")]
        {
            let string = Type::String(b"hello".to_vec());
            assert_eq!(Ok(b"hello".to_vec()), string.try_unwrap_string());
        }

        #[cfg(not(feature = "ascii"))]
        {
            let string = Type::String(String::from("hello"));
            assert_eq!(Ok(String::from("hello")), string.try_unwrap_string());
        }
    }

    #[test]
    fn to_object() {
        let mut object = HashMap::new();

        #[cfg(feature = "ascii")]
        let key = vec![b'a', b'c', b'd'];
        #[cfg(not(feature = "ascii"))]
        let key = String::from("acd");

        object.insert(key, Type::Nil);

        let object_type = Type::Object(object.clone());

        assert_eq!(Ok(object), object_type.try_unwrap_object());
    }

    #[test]
    fn to_array() {
        let mut array = WArray::new();
        array.push(Type::Nil);

        let array_type = Type::Array(array.clone());

        assert_eq!(Ok(array), array_type.try_unwrap_array());
    }

    #[test]
    fn to_bool() {
        let boolean = Type::Bool(false);

        assert_eq!(Ok(false), boolean.try_unwrap_bool());
    }

    // mmm tasty redundancy
    #[cfg(test)]
    mod serde_tests {
        use crate::types::{Type, WArray, WObject};

        fn create_stack() -> Vec<Type> {
            let mut stack = Vec::<Type>::new();

            stack.push(Type::Int(69));
            stack.push(Type::Int(-420));

            let mut object = WObject::new();
            #[cfg(feature = "ascii")]
            {
                object.insert(b"answer".to_vec(), Type::Uint(42));
                object.insert(b"nil".to_vec(), Type::Nil);
            }

            #[cfg(not(feature = "ascii"))]
            {
                object.insert(String::from("answer"), Type::Uint(42));
                object.insert(String::from("nil"), Type::Nil);
            }

            stack.push(Type::Object(object));

            let mut array = WArray::new();
            array.push(Type::Nil);
            array.push(Type::Int(-2));
            array.push(Type::Uint(3));
            array.push(Type::Float(3.5));

            stack.push(Type::Array(array));

            #[cfg(feature = "ascii")]
            let string = b"hello world".to_vec();
            #[cfg(not(feature = "ascii"))]
            let string = String::from("hello world");

            stack.push(Type::String(string));

            stack
        }

        #[test]
        fn toml_serde() {
            use toml;

            // FIXME: toml. unsupported Rust type
            // TOML might be incompatible with how serde
            // de/serializes this type of format
            let stack = create_stack();

            let toml_string = toml::to_string(&stack);
            assert!(toml_string.is_ok(), "{}", toml_string.unwrap_err().to_string());

            let toml_object = toml::from_str::<toml::Value>(toml_string.unwrap().as_str());
            assert!(toml_object.is_ok(), "{}", toml_object.unwrap_err().to_string());
            // unsupported Rust type??? WHICH ONE???????????????
        }

        #[test]
        fn json_serde() {
            use serde_json;

            let stack = create_stack();

            let json_string = serde_json::to_string(&stack);
            assert!(json_string.is_ok(), "{}", json_string.unwrap_err().to_string());

            let json_object = serde_json::from_str::<serde_json::Value>(json_string.unwrap().as_str());
            assert!(json_object.is_ok(), "{}", json_object.unwrap_err().to_string());
        }

        #[test]
        fn yaml_serde() {
            use serde_yaml;

            let stack = create_stack();

            let yaml_string = serde_yaml::to_string(&stack);
            assert!(yaml_string.is_ok(), "{}", yaml_string.unwrap_err().to_string());

            let yaml_object = serde_yaml::from_str::<serde_yaml::Value>(yaml_string.unwrap().as_str());
            assert!(yaml_object.is_ok(), "{}", yaml_object.unwrap_err().to_string());
        }
    }
}
