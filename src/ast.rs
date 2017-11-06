#[derive(Debug)]
pub enum OperatorType {
    Equal,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Object(String, Box<Token>),
    Equal(String, String),
}
