pub mod noise;
pub mod ast;

#[test]
fn noise() {
    //let out = noise::parse_Noise(r#"{"hello":"world!"}"#);
    let out = noise::parse_Noise(r#"{"hello": == "world!"}"#);
    //assert!(noise::parse_Noise(r#"{"hello":"world!"}"#).is_ok());
    println!("out: {:?}", out);
}
//fn calculator1() {
//    assert!(calculator1::parse_Term("22").is_ok());
//    assert!(calculator1::parse_Term("(22)").is_ok());
//    assert!(calculator1::parse_Term("((((22))))").is_ok());
//    assert!(calculator1::parse_Term("((22)").is_err());
//}
