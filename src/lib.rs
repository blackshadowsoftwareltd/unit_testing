// pub mod functions;
use rstest::*;

#[rstest]
fn is_equal_10(number_10: i32) {
    assert_eq!(number_10, 15);
}

#[rstest]
fn is_not_equal_10(number_10: i32) {
    assert_ne!(number_10, 15);
}

#[fixture]
pub fn number_10() -> i32 {
    10
}
