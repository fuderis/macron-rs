[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/regex)&ensp;
[![crates-io]](https://crates.io/crates/macron-regex)&ensp;
[![docs-rs]](https://docs.rs/macron-regex)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Regex Macro

Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples

```rust
let re = re!(r"^Hello, \w+!$");

assert!(re.is_match("Hello, World!"));

// with formatting:
let re = re!(r"^Hello, {}!$", "World");

assert!(re.is_match("Hello, World!"));

```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
