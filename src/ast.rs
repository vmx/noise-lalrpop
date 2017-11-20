#[derive(Debug)]
pub enum OperatorType {
    Equal,
    Greater,
    GreaterEqual,
    Intersect,
    Less,
    LessEqual,
    NotEqual,
    NotWordMatch(Option<u64>),
    WordMatch(Option<u64>),
}

#[derive(Debug)]
pub enum BoolType {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum OrderType {
    None,
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Array(Box<Token>),
    Bbox(f64, f64, f64, f64),
    Bind(String, Box<Token>),
    JsonArray(Vec<Token>),
    JsonBool(bool),
    JsonNull,
    JsonNumber(f64),
    JsonObject(String, Box<Token>),
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
    WordMatch(Option<String>, Option<u64>, Box<Token>),

    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),

    Order(Box<Option<Token>>, OrderType),

    All,
    Path(Vec<Token>),
    PathArray(Option<u64>),
    ReturnArray(Vec<Token>),
    ReturnBind(String, Box<Option<Token>>),
    ReturnObject(Vec<Token>),
    Default(Box<Token>, Box<Token>),

    GroupArray(Box<Token>),
    ArrayFlat(Box<Token>),
    Avg(Box<Token>),
    Count,
    Concat(Box<Token>, Option<String>),
    Group(Box<Token>, OrderType),
    Max(Box<Token>),
    MaxArray(Box<Token>),
    Min(Box<Token>),
    MinArray(Box<Token>),
    Score,
    Sum(Box<Token>),

    Limit(u64),

    Noise(Box<Token>, Vec<Token>, Box<Option<Token>>, Box<Option<Token>>),
}
