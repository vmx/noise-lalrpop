pub mod noise;
pub mod ast;

#[test]
fn noise() {
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": == "world"}"#)),
               r#"Ok(Equal("hello", "world"))"#);
    assert_eq!(format!("{:?}", noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#)),
               r#"Ok(Object("hello", Equal("nested", "world")))"#);

    let out = noise::parse_Noise(r#"find {"hello": {"nested": == "world"}}"#);
    println!("out: {:?}", out);
}
//fn calculator1() {
//    assert!(calculator1::parse_Term("22").is_ok());
//    assert!(calculator1::parse_Term("(22)").is_ok());
//    assert!(calculator1::parse_Term("((((22))))").is_ok());
//    assert!(calculator1::parse_Term("((22)").is_err());
//}
