[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/input)&ensp;
[![crates-io]](https://crates.io/crates/macron-input)&ensp;
[![docs-rs]](https://docs.rs/macron-input)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Read User Inputs

Reads user input lines from the console.

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples

```rust
let mut input = input!("Enter in order '0', '1', '2': ");

for i in 0..=2 {
    if let Ok(text) = input.next().unwrap() {
        assert_eq!(text, format!("{i}"));
    }
}
```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
