pub mod noise;
pub mod ast;

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), None))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Noise(Object("hello", Equal(Some("nested"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Noise(Object("hello", Object("deeply", Equal(Some("nested"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), None))"#);

    // Boolean
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Noise(And(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Noise(Or(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(Or(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" && "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} && {"another": == "one"}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} || {"another": == "one"} && {"third": == "thing"}"#)),
               r#"Ok(Noise(Or(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), None))"#);

    // Parenthesis
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one" || "third": == "thing")}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Or(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one", "third": == "thing")}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find ({"hello": == "world"})"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find ({"hello": == "world"} || {"another": == "one"}) && {"third": == "thing"}"#)),
               r#"Ok(Noise(And(Or(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), None))"#);

    // Arrays
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(Equal(Some("array"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Array(Equal(None, JsonString("world")))), None))"#);

    // Boost
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2}"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2, "another": == "one"}"#)),
               r#"Ok(Noise(And(Boost(2, Equal(Some("hello"), JsonString("world"))), Equal(Some("another"), JsonString("one"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {("hello": == "world")^2}"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}^2"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Equal(Some("nested"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(Equal(Some("array"), JsonString("world"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}^2]}"#)),
               r#"Ok(Noise(Object("hello", Array(Boost(2, Equal(Some("array"), JsonString("world"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one")))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"^2, "another": == "one"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(And(Boost(2, Equal(Some("array"), JsonString("world"))), Equal(Some("another"), JsonString("one"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(Equal(None, JsonString("world"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2]}"#)),
              r#"Ok(Noise(Object("hello", Array(Boost(2, Equal(None, JsonString("world"))))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2 || == "another"]}"#)),
              r#"Ok(Noise(Object("hello", Array(Or(Boost(2, Equal(None, JsonString("world"))), Equal(None, JsonString("another"))))), None))"#);

    // Bind variables
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": xyz::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("xyz", Array(Equal(None, JsonString("world"))))), None))"#);

    // Different types of values
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == null}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonNull), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == false}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonBool(false)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == true}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonBool(true)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 300}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonNumber(300)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 3.14}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonNumber(3.14)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "null"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("null")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "false"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("false")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "true"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("true")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "300"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("300")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "3.14"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("3.14")), None))"#);

    // Operators
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ~= "world"}"#)),
               r#"Ok(Noise(WordMatch(Some("hello"), JsonString("world")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": > 10}"#)),
               r#"Ok(Noise(Greater(Some("hello"), JsonNumber(10)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": >= 10}"#)),
               r#"Ok(Noise(GreaterEqual(Some("hello"), JsonNumber(10)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": < 10}"#)),
               r#"Ok(Noise(Less(Some("hello"), JsonNumber(10)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": <= 10}"#)),
               r#"Ok(Noise(LessEqual(Some("hello"), JsonNumber(10)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": && [10, 20, 30, 40]}"#)),
               r#"Ok(Noise(Intersect(Some("hello"), Bbox(10, 20, 30, 40)), None))"#);

    // Not
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find !{"hello": == "world"}"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": != "world"}"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !~= "world"}"#)),
               r#"Ok(Noise(Not(WordMatch(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find !({"hello": == "world"})"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !{"nested": == "world"}}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !({"nested": == "world"})}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": != "world"}}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [!{"array": == "world"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(Not(Equal(Some("array"), JsonString("world"))))), None))"#);

    // Return
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return ."#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(All)))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return .hello"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Path(".hello"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return .hello[]"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Path(".hello[]"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return .hello[0].nested"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Path(".hello[0].nested"))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return {"nested": .hello}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Object("nested", Path(".hello")))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return {"nested": {"deeper": .hello}}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Object("nested", Object("deeper", Path(".hello"))))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return [.hello]"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(ReturnArray([Path(".hello")]))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return [.hello, .another[5], .third.one]"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(ReturnArray([Path(".hello"), Path(".another[5]"), Path(".third.one")]))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return [.hello, {"nested": .one}]"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(ReturnArray([Path(".hello"), Object("nested", Path(".one"))]))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return {"nested": [.array]}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(Object("nested", ReturnArray([Path(".array")])))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return hello"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(ReturnBind("hello", None))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} return hello.nested[0]"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), Some(ReturnBind("hello", Some(Path(".nested[0]"))))))"#);

    let out = noise::parse_Noise(r#"find {"hello": == "world"} return ."#);
    println!("out: {:?}", out);
}
