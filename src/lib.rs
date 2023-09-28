pub mod functions;
use rstest::*;

#[rstest]
#[case(1, 10)]
#[case(2, 10)]
#[case(3, 10)]
#[case(4, 10)]
#[case(5, 10)]
fn fibonacci(#[case] input: u32, #[case] expected: u32) {
    assert_eq!(functions::sum_of_arr(input), expected);
}
