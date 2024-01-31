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
        let std_hash = hash_code(&"Hello, world!");
        let crc_32 = crc32_hash(&"Hello, world!");
        println!("{:?} == {:?}", std_hash, crc_32);
        assert_eq!(std_hash, crc_32 as u64);
    }

    fn hash_code<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    fn crc32_hash<T: std::hash::Hash>(data: &T) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        data.hash(&mut hasher); // Utilize the Hash trait for generic hashing
        hasher.finalize()
    }
}
