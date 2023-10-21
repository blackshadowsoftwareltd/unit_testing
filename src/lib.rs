pub mod functions;

// ? first way
#[tokio::test]
async fn test_sum1() {
    let handle = functions::sum_of_arr(10).await;
    assert_eq!(handle, 45);
}

// ? second way
#[tokio::test]
async fn test_sum() {
    let handle = tokio_test::task::spawn(functions::sum_of_arr(10)).await;
    assert_eq!(handle, 45);
}
