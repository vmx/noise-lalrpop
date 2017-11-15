pub mod noise;
pub mod ast;

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {}"#)),
               r#"Ok(Noise(All, [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), [], None, None))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Noise(Object("hello", Equal(Some("nested"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"deeply": {"nested": == "world"}}}"#)),
               r#"Ok(Noise(Object("hello", Object("deeply", Equal(Some("nested"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one"}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), [], None, None))"#);

    // Boolean
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Noise(And(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one", "third": == "thing"}"#)),
               r#"Ok(Noise(Or(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" || "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(Or(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world" && "another": == "one" || "third": == "thing"}"#)),
               r#"Ok(Noise(Or(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} && {"another": == "one"}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"} || {"another": == "one"} && {"third": == "thing"}"#)),
               r#"Ok(Noise(Or(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), [], None, None))"#);

    // Parenthesis
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one" || "third": == "thing")}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Or(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", ("another": == "one", "third": == "thing")}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), And(Equal(Some("another"), JsonString("one")), Equal(Some("third"), JsonString("thing")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find ({"hello": == "world"})"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find ({"hello": == "world"} || {"another": == "one"}) && {"third": == "thing"}"#)),
               r#"Ok(Noise(And(Or(Equal(Some("hello"), JsonString("world")), Equal(Some("another"), JsonString("one"))), Equal(Some("third"), JsonString("thing"))), [], None, None))"#);

    // Arrays
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(Equal(Some("array"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Array(Equal(None, JsonString("world")))), [], None, None))"#);

    // Boost
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2}"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"^2, "another": == "one"}"#)),
               r#"Ok(Noise(And(Boost(2, Equal(Some("hello"), JsonString("world"))), Equal(Some("another"), JsonString("one"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {("hello": == "world")^2}"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}^2"#)),
               r#"Ok(Noise(Boost(2, Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Equal(Some("nested"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(Equal(Some("array"), JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}^2]}"#)),
               r#"Ok(Noise(Object("hello", Array(Boost(2, Equal(Some("array"), JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world", "another": == "one"}]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one")))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"^2, "another": == "one"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(And(Boost(2, Equal(Some("array"), JsonString("world"))), Equal(Some("another"), JsonString("one"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]^2}"#)),
               r#"Ok(Noise(Object("hello", Boost(2, Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2]}"#)),
              r#"Ok(Noise(Object("hello", Array(Boost(2, Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"^2 || == "another"]}"#)),
              r#"Ok(Noise(Object("hello", Array(Or(Boost(2, Equal(None, JsonString("world"))), Equal(None, JsonString("another"))))), [], None, None))"#);

    // Bind variables
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": xyz::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("xyz", Array(Equal(None, JsonString("world"))))), [], None, None))"#);

    // Different types of values
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == null}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonNull), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == false}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonBool(false)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == true}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonBool(true)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 300}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonNumber(300)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == 3.14}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonNumber(3.14)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "null"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("null")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "false"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("false")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "true"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("true")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "300"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("300")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "3.14"}"#)),
              r#"Ok(Noise(Equal(Some("hello"), JsonString("3.14")), [], None, None))"#);

    // Operators
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ~= "world"}"#)),
               r#"Ok(Noise(WordMatch(Some("hello"), JsonString("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": > 10}"#)),
               r#"Ok(Noise(Greater(Some("hello"), JsonNumber(10)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": >= 10}"#)),
               r#"Ok(Noise(GreaterEqual(Some("hello"), JsonNumber(10)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": < 10}"#)),
               r#"Ok(Noise(Less(Some("hello"), JsonNumber(10)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": <= 10}"#)),
               r#"Ok(Noise(LessEqual(Some("hello"), JsonNumber(10)), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": && [10, 20, 30, 40]}"#)),
               r#"Ok(Noise(Intersect(Some("hello"), Bbox(10, 20, 30, 40)), [], None, None))"#);

    // Not
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find !{"hello": == "world"}"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": != "world"}"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !~= "world"}"#)),
               r#"Ok(Noise(Not(WordMatch(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find !({"hello": == "world"})"#)),
               r#"Ok(Noise(Not(Equal(Some("hello"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !{"nested": == "world"}}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !({"nested": == "world"})}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": != "world"}}"#)),
               r#"Ok(Noise(Object("hello", Not(Equal(Some("nested"), JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [!{"array": == "world"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(Not(Equal(Some("array"), JsonString("world"))))), [], None, None))"#);

    // Order
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc"#)),
               r#"Ok(Noise(All, [Order(Asc, None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order dsc"#)),
               r#"Ok(Noise(All, [Order(Dsc, None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello"#)),
               r#"Ok(Noise(All, [Order(None, Some(Path(".hello")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=["abc", true]"#)),
               r#"Ok(Noise(All, [Order(None, Some(Default(JsonArray([JsonString("abc"), JsonBool(true)]), Path(".hello"))))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc"#)),
               r#"Ok(Noise(All, [Order(Asc, Some(Path(".hello")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello dsc"#)),
               r#"Ok(Noise(All, [Order(Dsc, Some(Path(".hello")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=null dsc"#)),
               r#"Ok(Noise(All, [Order(Dsc, Some(Default(JsonNull, Path(".hello"))))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello, .world"#)),
               r#"Ok(Noise(All, [Order(None, Some(Path(".hello"))), Order(None, Some(Path(".world")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc, .world dsc"#)),
               r#"Ok(Noise(All, [Order(Asc, Some(Path(".hello"))), Order(Dsc, Some(Path(".world")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=1 asc, .world dsc"#)),
               r#"Ok(Noise(All, [Order(Asc, Some(Default(JsonNumber(1), Path(".hello")))), Order(Dsc, Some(Path(".world")))], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc, .world default={"some": "default"} dsc"#)),
               r#"Ok(Noise(All, [Order(Asc, Some(Path(".hello"))), Order(Dsc, Some(Default(JsonObject("some", JsonString("default")), Path(".world"))))], None, None))"#);

    // Return
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return ."#)),
               r#"Ok(Noise(All, [], Some(All), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello"#)),
               r#"Ok(Noise(All, [], Some(Path(".hello")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello[]"#)),
               r#"Ok(Noise(All, [], Some(Path(".hello[]")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello[0].nested"#)),
               r#"Ok(Noise(All, [], Some(Path(".hello[0].nested")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": .hello}"#)),
               r#"Ok(Noise(All, [], Some(Object("nested", Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": {"deeper": .hello}}"#)),
               r#"Ok(Noise(All, [], Some(Object("nested", Object("deeper", Path(".hello")))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path(".hello")])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello, .another[5], .third.one]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path(".hello"), Path(".another[5]"), Path(".third.one")])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello, {"nested": .one}]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path(".hello"), Object("nested", Path(".one"))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": [.array]}"#)),
               r#"Ok(Noise(All, [], Some(Object("nested", ReturnArray([Path(".array")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return hello"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("hello", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return hello.nested[0]"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("hello", Some(Path(".nested[0]")))), None))"#);

    // Return default value
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=null"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNull, Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=false"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonBool(false), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=true"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonBool(true), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=400"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNumber(400), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=-4.6"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNumber(-4.6), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default="world""#)),
               r#"Ok(Noise(All, [], Some(Default(JsonString("world"), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={"world": true}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObject("world", JsonBool(true)), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={"world": {"nested": 12}}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObject("world", JsonObject("nested", JsonNumber(12))), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=["world"]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonString("world")]), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=["world", null]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonString("world"), JsonNull]), Path(".hello"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=[{"world": null}]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonObject("world", JsonNull)]), Path(".hello"))), None))"#);

    // Limit
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} limit 10"#)),
               r#"Ok(Noise(All, [], None, Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc limit 10"#)),
               r#"Ok(Noise(All, [Order(Asc, None)], None, Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc return . limit 10"#)),
               r#"Ok(Noise(All, [Order(Asc, None)], Some(All), Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return . limit 10"#)),
               r#"Ok(Noise(All, [], Some(All), Some(Limit(10))))"#);

    // Group
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Path(".hello"), None)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello dsc)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Path(".hello"), Dsc)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello default=2)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Default(JsonNumber(2), Path(".hello")), None)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello default=2 asc)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Default(JsonNumber(2), Path(".hello")), Asc)])), None))"#);

    let out = noise::parse_Noise(r#"find {"hello": == "world"} return ."#);
    println!("out: {:?}", out);
}
