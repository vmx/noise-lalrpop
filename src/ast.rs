#[derive(Debug)]
pub enum OperatorType {
    Equal,
    Greater,
    GreaterEqual,
    Intersect,
    Less,
    LessEqual,
    NotEqual,
    NotWordMatch,
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
    Bbox(f64, f64, f64, f64),
    Bind(String, Box<Token>),
    JsonBool(bool),
    JsonNull,
    JsonNumber(f64),
    JsonString(String),
    Literal(String),
    Not(Box<Token>),
    Object(String, Box<Token>),
    Boost(u64, Box<Token>),

    // If the key is `None` then it's an operator within an array
    Equal(Option<String>, Box<Token>),
    Greater(Option<String>, Box<Token>),
    GreaterEqual(Option<String>, Box<Token>),
    Intersect(Option<String>, Box<Token>),
    Less(Option<String>, Box<Token>),
    LessEqual(Option<String>, Box<Token>),
    WordMatch(Option<String>, Box<Token>),

    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
}
