[![crates.io](https://img.shields.io/crates/v/unescape.svg)](https://crates.io/crates/unescape) [![Build Status](https://travis-ci.org/saghm/unescape-rs.svg)](https://travis-ci.org/saghm/unescape-rs)

unescape
====================================================================================================================================

"Unescapes" (JavaScript) strings with escape sequences written out as literal
characters.

```
extern crate unescape;

use unescape::unescape;

fn main() {
  println!("{}", unescape(r"\u0038\u0039").unwrap()); // prints out "89"
}
```
