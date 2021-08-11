use sai::derivation::SelfAddressing;

#[test]
fn test_derive() {
    let data = "hello there";
    let sai = SelfAddressing::Blake3_256.derive(data.as_bytes());

    assert_eq!(format!("{}", sai), "E2bCqepXGid_9s1nSEyKk4kljbl3GULx5JjkFvIwJ8sk");
    assert!(sai.verify_binding(data.as_bytes()));
    assert!(!sai.verify_binding("wrong data".as_bytes()));
}