pub async fn sum_of_arr(l: i32) -> i32 {
    let x = (0..l).sum();
    println!("====sum of arr is {}", x);
    x
}
