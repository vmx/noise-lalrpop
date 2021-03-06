use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod noise;
pub mod ast;

pub fn main() {
    for argument in env::args().skip(1) {
        println!("{}", argument);
        parse_file(argument);
    }
}

fn parse_file(filename: String) {
    let error = format!("cannot open file: {}", filename);
    let file = File::open(filename).expect(&error);
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        if line.starts_with("#") {
            continue;
        }
        println!("{}", line);
        let parsed = noise::parse_Noise(&line);
        match parsed {
            Ok(_) => println!("ok"),
            Err(_) => {
                println!("{:?}", parsed);
                std::process::exit(1);
            },
        }
    }
}

#[test]
fn noise() {
    // Simple
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {}"#)),
               r#"Ok(Noise(All, [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {hello: == "world"}"#)),
               r#"Ok(Noise(Equal(Some("hello"), JsonString("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hell \"escaped\"": == "world"}"#)),
               r#"Ok(Noise(Equal(Some("hell \\\"escaped\\\""), JsonString("world")), [], None, None))"#);

    // Nested
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Noise(Object("hello", Equal(Some("nested"), JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {hello: {nested: == "world"}}"#)),
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
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [{"array": == "world"}, {"another": == "one"}]}"#)),
               r#"Ok(Noise(Object("hello", Array(And(Equal(Some("array"), JsonString("world")), Equal(Some("another"), JsonString("one"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Array(Equal(None, JsonString("world")))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": [[== "world"]]}"#)),
               r#"Ok(Noise(Object("hello", Array(Array(Equal(None, JsonString("world"))))), [], None, None))"#);

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
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": array_flat::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("array_flat", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": find::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("find", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": order::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("order", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": return::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("return", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": limit::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("limit", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": asc::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("asc", Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": desc::[== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Bind("desc", Array(Equal(None, JsonString("world"))))), [], None, None))"#);

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
               r#"Ok(Noise(WordMatch(Some("hello"), None, JsonString("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ~34= "world"}"#)),
               r#"Ok(Noise(WordMatch(Some("hello"), Some(34), JsonString("world")), [], None, None))"#);
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
               r#"Ok(Noise(Not(WordMatch(Some("hello"), None, JsonString("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": !~123= "world"}"#)),
               r#"Ok(Noise(Not(WordMatch(Some("hello"), Some(123), JsonString("world"))), [], None, None))"#);
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
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ![== "world"]}"#)),
               r#"Ok(Noise(Object("hello", Not(Array(Equal(None, JsonString("world"))))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world", !("another": == "one")}"#)),
               r#"Ok(Noise(And(Equal(Some("hello"), JsonString("world")), Not(Equal(Some("another"), JsonString("one")))), [], None, None))"#);

    // Order
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc"#)),
               r#"Ok(Noise(All, [Order(None, Asc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order desc"#)),
               r#"Ok(Noise(All, [Order(None, Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=["abc", true]"#)),
               r#"Ok(Noise(All, [Order(Some(Default(JsonArray([JsonString("abc"), JsonBool(true)]), Path([JsonString("hello")]))), None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), Asc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello desc"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=null desc"#)),
               r#"Ok(Noise(All, [Order(Some(Default(JsonNull, Path([JsonString("hello")]))), Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello, .world"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), None), Order(Some(Path([JsonString("world")])), None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc, .world desc"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), Asc), Order(Some(Path([JsonString("world")])), Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello default=1 asc, .world desc"#)),
               r#"Ok(Noise(All, [Order(Some(Default(JsonNumber(1), Path([JsonString("hello")]))), Asc), Order(Some(Path([JsonString("world")])), Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order .hello asc, .world default={"some": "default"} desc"#)),
               r#"Ok(Noise(All, [Order(Some(Path([JsonString("hello")])), Asc), Order(Some(Default(JsonObject("some", JsonString("default")), Path([JsonString("world")]))), Desc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order score()"#)),
               r#"Ok(Noise(All, [Order(Some(Score), None)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order score() asc"#)),
               r#"Ok(Noise(All, [Order(Some(Score), Asc)], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order score() desc"#)),
               r#"Ok(Noise(All, [Order(Some(Score), Desc)], None, None))"#);

    // Return
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return ."#)),
               r#"Ok(Noise(All, [], Some(All), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello"#)),
               r#"Ok(Noise(All, [], Some(Path([JsonString("hello")])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello[]"#)),
               r#"Ok(Noise(All, [], Some(Path([JsonString("hello"), PathArray(None)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello[0].nested"#)),
               r#"Ok(Noise(All, [], Some(Path([JsonString("hello"), PathArray(Some(0)), JsonString("nested")])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": .hello}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("nested", Path([JsonString("hello")]))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": {"deeper": .hello}}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("nested", ReturnObject([Object("deeper", Path([JsonString("hello")]))]))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": {"deeper": .hello, "one": .world}}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("nested", ReturnObject([Object("deeper", Path([JsonString("hello")])), Object("one", Path([JsonString("world")]))]))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path([JsonString("hello")])])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello, .another[5], .third.one]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path([JsonString("hello")]), Path([JsonString("another"), PathArray(Some(5))]), Path([JsonString("third"), JsonString("one")])])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [.hello, {"nested": .one}]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Path([JsonString("hello")]), ReturnObject([Object("nested", Path([JsonString("one")]))])])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"hello": [{"array": .nested}]}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("hello", ReturnArray([ReturnObject([Object("array", Path([JsonString("nested")]))])]))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"nested": [.array]}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("nested", ReturnArray([Path([JsonString("array")])]))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return ._id"#)),
               r#"Ok(Noise(All, [], Some(Path([JsonString("_id")])), None))"#);

    // Return bind variables
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return hello"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("hello", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return hello.nested[0]"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("hello", Some(Path([JsonString("nested"), PathArray(Some(0))])))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return array"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("array", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return find"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("find", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return order"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("order", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return return"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("return", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return limit"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("limit", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return asc"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("asc", None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return desc"#)),
               r#"Ok(Noise(All, [], Some(ReturnBind("desc", None)), None))"#);

    // Return JSON
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return false"#)),
               r#"Ok(Noise(All, [], Some(JsonBool(false)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return true"#)),
               r#"Ok(Noise(All, [], Some(JsonBool(true)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return null"#)),
               r#"Ok(Noise(All, [], Some(JsonNull), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return 123"#)),
               r#"Ok(Noise(All, [], Some(JsonNumber(123)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return 123.456"#)),
               r#"Ok(Noise(All, [], Some(JsonNumber(123.456)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return "true""#)),
               r#"Ok(Noise(All, [], Some(JsonString("true")), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {}"#)),
              r#"Ok(Noise(All, [], Some(ReturnObject([])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return { }"#)),
              r#"Ok(Noise(All, [], Some(ReturnObject([])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {"hello": true}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("hello", JsonBool(true))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {hello: true}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("hello", JsonBool(true))])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return []"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [ ]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [null, 12]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([JsonNull, JsonNumber(12)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return {hello: true, another: "one"}"#)),
               r#"Ok(Noise(All, [], Some(ReturnObject([Object("hello", JsonBool(true)), Object("another", JsonString("one"))])), None))"#);

    // Return default value
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=null"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNull, Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=false"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonBool(false), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=true"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonBool(true), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=400"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNumber(400), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=-4.6"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonNumber(-4.6), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default="world""#)),
               r#"Ok(Noise(All, [], Some(Default(JsonString("world"), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObjectEmpty, Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={ }"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObjectEmpty, Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={"world": true}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObject("world", JsonBool(true)), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={"world": {"nested": 12}}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObject("world", JsonObject("nested", JsonNumber(12))), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default={world: {nested: 12}}"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonObject("world", JsonObject("nested", JsonNumber(12))), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=["world"]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonString("world")]), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=["world", null]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonString("world"), JsonNull]), Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return .hello default=[{"world": null}]"#)),
               r#"Ok(Noise(All, [], Some(Default(JsonArray([JsonObject("world", JsonNull)]), Path([JsonString("hello")]))), None))"#);

    // Limit
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} limit 10"#)),
               r#"Ok(Noise(All, [], None, Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc limit 10"#)),
               r#"Ok(Noise(All, [Order(None, Asc)], None, Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} order asc return . limit 10"#)),
               r#"Ok(Noise(All, [Order(None, Asc)], Some(All), Some(Limit(10))))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return . limit 10"#)),
               r#"Ok(Noise(All, [], Some(All), Some(Limit(10))))"#);

    // Group
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Path([JsonString("hello")]), None)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello desc)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Path([JsonString("hello")]), Desc)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello default=2)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Default(JsonNumber(2), Path([JsonString("hello")])), None)])), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return [group(.hello default=2 asc)]"#)),
               r#"Ok(Noise(All, [], Some(ReturnArray([Group(Default(JsonNumber(2), Path([JsonString("hello")])), Asc)])), None))"#);

    // Aggregations
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return array(.hello)"#)),
               r#"Ok(Noise(All, [], Some(GroupArray(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return array_flat(.hello)"#)),
               r#"Ok(Noise(All, [], Some(ArrayFlat(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return avg(.hello)"#)),
               r#"Ok(Noise(All, [], Some(Avg(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return count()"#)),
               r#"Ok(Noise(All, [], Some(Count), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return concat(.hello)"#)),
               r#"Ok(Noise(All, [], Some(Concat(Path([JsonString("hello")]), None)), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return concat(.hello sep="|")"#)),
               r#"Ok(Noise(All, [], Some(Concat(Path([JsonString("hello")]), Some("|"))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return max(.hello)"#)),
               r#"Ok(Noise(All, [], Some(Max(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return max_array(.hello)"#)),
               r#"Ok(Noise(All, [], Some(MaxArray(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return min(.hello)"#)),
               r#"Ok(Noise(All, [], Some(Min(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return min_array(.hello)"#)),
               r#"Ok(Noise(All, [], Some(MinArray(Path([JsonString("hello")]))), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return score()"#)),
               r#"Ok(Noise(All, [], Some(Score), None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {} return sum(.hello)"#)),
               r#"Ok(Noise(All, [], Some(Sum(Path([JsonString("hello")]))), None))"#);

    // Parameters
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == @world}"#)),
               r#"Ok(Noise(Equal(Some("hello"), Parameter("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == @world}}"#)),
               r#"Ok(Noise(Object("hello", Equal(Some("nested"), Parameter("world"))), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": ~34= @world}"#)),
               r#"Ok(Noise(WordMatch(Some("hello"), Some(34), Parameter("world")), [], None, None))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": > @world}"#)),
               r#"Ok(Noise(Greater(Some("hello"), Parameter("world")), [], None, None))"#);
}
