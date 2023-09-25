pub mod functions;

#[warn(dead_code)]
fn main() {
    print!("{}", functions::concat_tow_str("Hellow ", "World"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_tow_str() {
        assert_eq!(
            functions::concat_tow_str("Hellow ", "World"),
            "Hellow World"
        );
    }
}
