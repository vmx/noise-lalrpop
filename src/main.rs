pub mod noise;
pub mod ast;

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Equal(Some("hello"), JsonString("world")))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Object("hello", Equal(Some("nested"), JsonString("world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Object("hello", Object("deeply", Equal(Some("nested"), JsonString("world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))))"#);

    // Boolean
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(And(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Or(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(Or(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" && "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))))"#);

    // Parenthesis
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one" || "third": == "thing")}"#)),
               r#"Ok(And(Equal(Some("hello"), JsonString("world")), Or(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one", "third": == "thing")}"#)),
               r#"Ok(And(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))))"#);

    // Arrays
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]}"#)),
               r#"Ok(Object("hello", Array(Equal(Some("array"), JsonString("world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]}"#)),
               r#"Ok(Object("hello", Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]}"#)),
               r#"Ok(Object("hello", Array(Equal(None, JsonString("world")))))"#);

    // Boost
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2}"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), JsonString("world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2, "another": == "one"}"#)),
               r#"Ok(And(Boost(2, Equal(Some("hello"), JsonString("world"))), Equal(Some("another"), JsonString("one"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {("hello": == "world")^2}"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), JsonString("world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}^2"#)),
               r#"Ok(Boost(2, Equal(Some("hello"), JsonString("world"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}^2}"#)),
               r#"Ok(Object("hello", Boost(2, Equal(Some("nested"), JsonString("world")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(Equal(Some("array"), JsonString("world"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}^2]}"#)),
               r#"Ok(Object("hello", Array(Boost(2, Equal(Some("array"), JsonString("world"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one")))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"^2, "another": == "one"}]}"#)),
               r#"Ok(Object("hello", Array(And(Boost(2, Equal(Some("array"), JsonString("world"))), Equal(Some("another"), JsonString("one"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]^2}"#)),
               r#"Ok(Object("hello", Boost(2, Array(Equal(None, JsonString("world"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2]}"#)),
              r#"Ok(Object("hello", Array(Boost(2, Equal(None, JsonString("world"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2 || == "another"]}"#)),
              r#"Ok(Object("hello", Array(Or(Boost(2, Equal(None, JsonString("world"))), Equal(None, JsonString("another"))))))"#);

    // Bind variables
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": xyz::[== "world"]}"#)),
               r#"Ok(Object("hello", Bind("xyz", Array(Equal(None, JsonString("world"))))))"#);

    // Different types of values
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == null}"#)),
               r#"Ok(Equal(Some("hello"), JsonNull))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == false}"#)),
               r#"Ok(Equal(Some("hello"), JsonBool(false)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == true}"#)),
               r#"Ok(Equal(Some("hello"), JsonBool(true)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 300}"#)),
              r#"Ok(Equal(Some("hello"), JsonNumber(300)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 3.14}"#)),
               r#"Ok(Equal(Some("hello"), JsonNumber(3.14)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "null"}"#)),
              r#"Ok(Equal(Some("hello"), JsonString("null")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "false"}"#)),
              r#"Ok(Equal(Some("hello"), JsonString("false")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "true"}"#)),
              r#"Ok(Equal(Some("hello"), JsonString("true")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "300"}"#)),
              r#"Ok(Equal(Some("hello"), JsonString("300")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "3.14"}"#)),
              r#"Ok(Equal(Some("hello"), JsonString("3.14")))"#);

    // Operators
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ~= "world"}"#)),
               r#"Ok(WordMatch(Some("hello"), JsonString("world")))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": > 10}"#)),
               r#"Ok(Greater(Some("hello"), JsonNumber(10)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": >= 10}"#)),
               r#"Ok(GreaterEqual(Some("hello"), JsonNumber(10)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": < 10}"#)),
               r#"Ok(Less(Some("hello"), JsonNumber(10)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": <= 10}"#)),
               r#"Ok(LessEqual(Some("hello"), JsonNumber(10)))"#);

    let out = noise::parse_Noise(r#"find {"hello": ~= "world"}"#);
    println!("out: {:?}", out);
}
