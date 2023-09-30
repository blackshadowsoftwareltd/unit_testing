pub mod functions;
use rstest::*;
use rstest_reuse::{self, *};

// ? custom template for reuse test case
#[template]
#[rstest]
#[case(3, 10)]
#[case(5, 10)]
#[case(7, 10)]
fn test_template(#[case] a: i32, #[case] b: i32) {}

// ? applying a custom template on a test function
#[apply(test_template)]
fn is_valid(a: i32, b: i32) {
    assert_eq!(functions::sum_of_arr(a), b);
}

// ? applying a custom template on another test function
#[apply(test_template)]
fn is_equal(a: i32, b: i32) {
    assert_eq!(functions::sum_of_arr(a), b)
}
