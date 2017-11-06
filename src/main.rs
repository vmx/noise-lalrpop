pub mod noise;
pub mod ast;

#[test]
fn noise() {
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Equal("hello", "world"))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Object("hello", Equal("nested", "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Object("hello", Object("deeply", Equal("nested", "world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(And(Equal("hello", "world"), Equal("another", "one")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(And(And(Equal("hello", "world"), Equal("another", "one")), Equal("third", "thing")))"#);

    let out = noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#);
    println!("out: {:?}", out);
}
