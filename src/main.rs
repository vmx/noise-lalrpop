pub mod noise;
pub mod ast;

use ast::Token;

#[test]
fn noise() {
    assert_eq!(noise::parse_Noise(r#"find {"hello": == "world"}"#).unwrap(),
               Token::Equal("hello".to_string(), "world".to_string()));
    assert_eq!(noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#).unwrap(),
               Token::Object("hello".to_string(),
                             Box::new(Token::Equal("nested".to_string(), "world".to_string()))));

    let out = noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#);
    println!("out: {:?}", out);
}
//fn calculator1() {
//    assert!(calculator1::parse_Term("22").is_ok());
//    assert!(calculator1::parse_Term("(22)").is_ok());
//    assert!(calculator1::parse_Term("((((22))))").is_ok());
//    assert!(calculator1::parse_Term("((22)").is_err());
//}
