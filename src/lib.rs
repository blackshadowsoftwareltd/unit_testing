pub mod functions;

/// Returns the length of the given string.
/// ```
/// use unit_testing::get_string_len;
/// assert_eq!(get_string_len("Hello World".to_string()), 11);
/// assert_eq!(get_string_len("".to_string()), 10); // ! main' panicked at 'assertion failed: `(left == right)` left: `0`, right: `10`', src/lib.rs:7:1
/// ```

pub fn get_string_len(s: String) -> usize {
    if s.is_empty() {
        return 0;
    }
    s.len()
}
