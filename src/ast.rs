#[derive(Debug)]
pub enum OperatorType {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    WordMatch,
}

#[derive(Debug)]
pub enum BoolType {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Array(Box<Token>),
    Bind(String, Box<Token>),
    JsonBool(bool),
    JsonNull,
    JsonNumber(f64),
    JsonString(String),
    Literal(String),
    Object(String, Box<Token>),
    Boost(u64, Box<Token>),

    // If the key is `None` then it's an operator within an array
    Equal(Option<String>, Box<Token>),
    Greater(Option<String>, Box<Token>),
    GreaterEqual(Option<String>, Box<Token>),
    Less(Option<String>, Box<Token>),
    LessEqual(Option<String>, Box<Token>),
    WordMatch(Option<String>, Box<Token>),

    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
}
