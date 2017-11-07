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
    Literal(String),
    Object(String, Box<Token>),
    Boost(u64, Box<Token>),

    Equal(String, String),

    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
}
