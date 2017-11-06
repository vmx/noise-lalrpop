#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal(String, String),
}

#[derive(Debug)]
pub enum OperatorType {
    Equal,
}
