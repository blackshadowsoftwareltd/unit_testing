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

mod _hash_test {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[tokio::test]
    async fn do_hash() {
        let hello_world = hash_code(&"Hello, world!");
        println!("Hello, world! hash: {:x}", hello_world);
        assert_eq!(hello_world, hash_code(&"Hello, world!"));
    }

    fn hash_code<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
