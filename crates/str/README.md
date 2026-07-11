[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/str)&ensp;
[![crates-io]](https://crates.io/crates/macron-str)&ensp;
[![docs-rs]](https://docs.rs/macron-str)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# String Macro

Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).


## Examples

```rust
// simple string:
let s = str!("Hello, World!");

assert_eq!(s, "Hello, World!");

// from integer:
let s = str!(10);

assert_eq!(s, "10");

// from refference:
let r = 10.2;
let s = str!(r);

assert_eq!(s, "10.2");

// string formatting with arguments:
let s = str!("Hello, {}!", "World");

assert_eq!(s, "Hello, World!");

// string formatting with named arguments:
let name = "World";
let s = str!("Hello, {name}!");

assert_eq!(s, "Hello, World!");
```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
