pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[warn(dead_code)]
pub fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}
