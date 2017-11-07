#[derive(Debug)]
pub enum OperatorType {
    Equal,
}

#[derive(Debug)]
pub enum BoolType {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Array(Box<Token>),
    Literal(String),
    Object(String, Box<Token>),
    Boost(u64, Box<Token>),

    // If the key is `None` then it's an operator within an array
    Equal(Option<String>, String),

    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
}
