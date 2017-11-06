pub mod noise;
pub mod ast;

#[test]
fn noise() {
    assert_eq!(noise::parse_Noise(r#"find {"hello": == "world"}"#).unwrap(),
    ast::Operator::Equal("hello".to_string(), "world".to_string()));
    let out = noise::parse_Noise(r#"find {"hello": == "world"}"#);
    println!("out: {:?}", out);
}
//fn calculator1() {
//    assert!(calculator1::parse_Term("22").is_ok());
//    assert!(calculator1::parse_Term("(22)").is_ok());
//    assert!(calculator1::parse_Term("((((22))))").is_ok());
//    assert!(calculator1::parse_Term("((22)").is_err());
//}
