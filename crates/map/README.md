[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/map)&ensp;
[![crates-io]](https://crates.io/crates/macron-map)&ensp;
[![docs-rs]](https://docs.rs/macron-map)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Map Collection Parser

The key-value collection parser.

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples

```rust
let (k, v) = ("one", 1);
    
let map = parse_map! {
    k => v,
    "two": 2,
    "three" => 3,
    "four": 4,
};

assert_eq!(map, [("one", 1), ("two", 2), ("three", 3), ("four", 4)])
```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
