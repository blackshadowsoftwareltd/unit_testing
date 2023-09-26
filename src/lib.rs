pub mod functions;

#[test]
fn test_positive_number() {
    assert!(functions::validate(100).is_ok());
}

#[test]
fn test_negative_number() {
    assert!(functions::validate(-100).is_ok());
}
