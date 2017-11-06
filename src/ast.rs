#[derive(Debug)]
pub enum OperatorType {
    Equal,
}

#[derive(Debug)]
pub enum BoolType {
    And,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Object(String, Box<Token>),

    Equal(String, String),

    And(Box<Token>, Box<Token>),
}
