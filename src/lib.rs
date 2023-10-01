pub mod functions;
use rstest::*;

#[rstest]
#[case(5, 10)]
#[should_panic] // ! should_panic ingnore getting panic
#[case(10, 10)]
async fn async_test_func(#[case] a: i32, #[case] b: i32) {
    assert_eq!(functions::sum_of_arr(a).await, b);
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[fixture]
async fn base() -> i32 {
    functions::sum_of_arr(10).await
}

#[rstest]
#[case(5,async {9})]
#[case(15,async{3})]
async fn my_async_test(
    // ------------------- first case
    #[future] base: i32,
    // ------------------- second case
    #[case] expected: i32,
    // ------------------- third case
    #[future]
    #[case]
    div: i32,
) {
    assert_eq!(expected, base.await / div.await);
}
