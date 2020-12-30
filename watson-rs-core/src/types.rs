pub type WatsonCharacter = u8;
pub type WChar = WatsonCharacter;
pub type WatsonString = Vec<WChar>;
pub type WString = WatsonString;
pub enum TokenType {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(WString),
    Object((WString, Box<TokenType>)),
    Array(Vec<TokenType>),
    Bool(bool),
    Nil,
}

impl TokenType {
    pub fn int_to_wchar(int: i64) -> WChar {
        (int & !0xff) as WChar
    }    
}
