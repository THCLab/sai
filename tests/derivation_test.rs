use said::derivation::SelfAddressing;

#[test]
fn test_derive() {
    let data = "hello there";
    let sai = SelfAddressing::Blake3_256.derive(data.as_bytes());

    assert_eq!(
        format!("{}", sai),
        "ENmwqnqVxonf_bNZ0hMipOJJY25dxlC8eSY5BbyMCfLJ"
    );
    assert!(sai.verify_binding(data.as_bytes()));
    assert!(!sai.verify_binding("wrong data".as_bytes()));
}
