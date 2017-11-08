pub mod noise;
pub mod ast;

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Equal(Some("hello"), "world"))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Object("hello", Equal(Some("nested"), "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Object("hello", Object("deeply", Equal(Some("nested"), "world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(And(Equal(Some("hello"), "world"), Equal(Some("another"), "one")))"#);

    // Boolean
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(And(And(Equal(Some("hello"), "world"), Equal(Some("another"), "one")), Equal(Some("third"), "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal(Some("hello"), "world"), Equal(Some("another"), "one")), Equal(Some("third"), "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Or(Equal(Some("hello"), "world"), And(Equal(Some("another"), "one"), Equal(Some("third"), "thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(Or(Equal(Some("hello"), "world"), Equal(Some("another"), "one")), Equal(Some("third"), "thing")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" && "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal(Some("hello"), "world"), Equal(Some("another"), "one")), Equal(Some("third"), "thing")))"#);

    // Parenthesis
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one" || "third": == "thing")}"#)),
               r#"Ok(And(Equal(Some("hello"), "world"), Or(Equal(Some("another"), "one"), Equal(Some("third"), "thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one", "third": == "thing")}"#)),
               r#"Ok(And(Equal(Some("hello"), "world"), And(Equal(Some("another"), "one"), Equal(Some("third"), "thing"))))"#);

    // Arrays
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]}"#)),
               r#"Ok(Object("hello", Array(Equal(Some("array"), "world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]}"#)),
               r#"Ok(Object("hello", Array(And(Equal(Some("array"), "world"), Equal(Some("another"), "one")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]}"#)),
               r#"Ok(Object("hello", Array(Equal(None, "world"))))"#);

    // Boost
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2}"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2, "another": == "one"}"#)),
               r#"Ok(And(Boost(2, Equal(Some("hello"), "world")), Equal(Some("another"), "one")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {("hello": == "world")^2}"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}^2"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), "world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}^2}"#)),
               r#"Ok(Object("hello", Boost(2, Equal(Some("nested"), "world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(Equal(Some("array"), "world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}^2]}"#)),
               r#"Ok(Object("hello", Array(Boost(2, Equal(Some("array"), "world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(And(Equal(Some("array"), "world"), Equal(Some("another"), "one"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"^2, "another": == "one"}]}"#)),
               r#"Ok(Object("hello", Array(And(Boost(2, Equal(Some("array"), "world")), Equal(Some("another"), "one")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(Equal(None, "world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2]}"#)),
              r#"Ok(Object("hello", Array(Boost(2, Equal(None, "world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2 || == "another"]}"#)),
              r#"Ok(Object("hello", Array(Or(Boost(2, Equal(None, "world")), Equal(None, "another")))))"#);

    // Bind variables
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": xyz::[== "world"]}"#)),
               r#"Ok(Object("hello", Bind("xyz", Array(Equal(None, "world")))))"#);

    let out = noise::parse_Noise(r#"find {"hello": xyz::[== "world"]}"#);
    println!("out: {:?}", out);
}
