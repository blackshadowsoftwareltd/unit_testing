pub mod functions;

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
pub mod test_module {
    use super::*;

    #[test]
    pub fn test_add() {
        assert_eq!(functions::add(1, 2), 3)
    }

    #[test]
    pub fn test_bad_add() {
        assert_eq!(functions::bad_add(1, 2), 3)
    }
}
