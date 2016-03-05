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
    assert_some_string!("First line\nSecond line", r"First line\nSecond line");
    assert_some_string!("First line\r\nSecond line", r"First line\r\nSecond line");
    assert_some_string!("Unindented\tIndented", r"Unindented\tIndented");
    assert_some_string!("'This is singly quoted!'", r"\'This is singly quoted!\'");
    assert_some_string!("\"This is doubly quoted!\"", r#"\"This is doubly quoted!\""#);
    assert_some_string!("This is one backslash: \\", r"This is one backslash: \\");
}

#[test]
fn unicode_chars() {
    assert_some_string!("\n", r"\u000A");
    assert_some_string!("\u{1234}", r"\u1234");
}

#[test]
fn byte_chars() {
    assert_some_string!("\n", r"\x0A");
    assert_some_string!("\x23", r"\x23");
}

#[test]
fn octal_chars() {
    assert_some_string!("\n", r"\12");
    assert_some_string!("\u{00C4}", r"\304");
}
