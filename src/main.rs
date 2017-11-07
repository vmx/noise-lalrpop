pub mod noise;
pub mod ast;

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Equal("hello", "world"))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Object("hello", Equal("nested", "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Object("hello", Object("deeply", Equal("nested", "world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(And(Equal("hello", "world"), Equal("another", "one")))"#);

    // Boolean
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(And(And(Equal("hello", "world"), Equal("another", "one")), Equal("third", "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal("hello", "world"), Equal("another", "one")), Equal("third", "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Or(Equal("hello", "world"), And(Equal("another", "one"), Equal("third", "thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(Or(Equal("hello", "world"), Equal("another", "one")), Equal("third", "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" && "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal("hello", "world"), Equal("another", "one")), Equal("third", "thing")))"#);

    // Parenthesis
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one" || "third": == "thing")}"#)),
               r#"Ok(And(Equal("hello", "world"), Or(Equal("another", "one"), Equal("third", "thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one", "third": == "thing")}"#)),
               r#"Ok(And(Equal("hello", "world"), And(Equal("another", "one"), Equal("third", "thing"))))"#);

    // Boost
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2}"#)),
               r#"Ok(Boost(2, Equal("hello", "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2, "another": == "one"}"#)),
               r#"Ok(And(Boost(2, Equal("hello", "world")), Equal("another", "one")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {("hello": == "world")^2}"#)),
               r#"Ok(Boost(2, Equal("hello", "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}^2"#)),
               r#"Ok(Boost(2, Equal("hello", "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}^2}"#)),
               r#"Ok(Object("hello", Boost(2, Equal("nested", "world"))))"#);

    let out = noise::parse_Noise(r#"find {"hello": == "world"}^2"#);
    println!("out: {:?}", out);
}
