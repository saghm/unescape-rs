extern crate unescape;

use unescape::unescape;

macro_rules! assert_some_string {
    ($s1:expr, $s2:expr) => {
        assert_eq!(Some(String::from($s1)), unescape($s2));
    }
}

#[test]
fn no_escapes() {
    assert_some_string!("", "");
    assert_some_string!("hello", "hello");
    assert_some_string!("these are some pretty crazy strings here", "these are some pretty crazy strings here");
}

#[test]
fn control_chars() {
    assert_some_string!("First line\nSecond line", "First line\\nSecond line");
    assert_some_string!("First line\r\nSecond line", "First line\\r\\nSecond line");
    assert_some_string!("Unindented\tIndented", "Unindented\tIndented");
    assert_some_string!("'This is singly quoted!'", "\\'This is singly quoted!\\'");
    assert_some_string!("\"This is doubly quoted!\"", "\\\"This is doubly quoted!\\\"");
    assert_some_string!("This is one backslash: \\", "This is one backslash: \\\\");
}

#[test]
fn unicode_chars() {
    assert_some_string!("\n", "\\u000A");
    assert_some_string!("\u{1234}", "\\u1234");
}

#[test]
fn byte_chars() {
    assert_some_string!("\n", "\\x0A");
    assert_some_string!("\x23", "\\x23");
}

#[test]
fn octal_chars() {
    assert_some_string!("\n", "\\12");
    assert_some_string!("\u{00C4}", "\\304");
}
