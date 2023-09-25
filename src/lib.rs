pub mod functions;

#[test]
fn test_empty() {
    assert_eq!(functions::first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(functions::first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(functions::first_word("Hello World!"), "Hello");
}

#[test]
fn test_random_words() {
    assert_eq!(
        functions::first_word("This is unit testing in rust language."),
        "Hello"
    );
}
