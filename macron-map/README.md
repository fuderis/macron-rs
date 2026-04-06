[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-map)&ensp;
[![crates-io]](https://crates.io/crates/macron-map)&ensp;
[![docs-rs]](https://docs.rs/macron-map)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Map Collection Parser

## Introduction:

The key-value collection parser.

P.s.: More useful macros you can find [here](https://docs.rs/macron).


## Examples:

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

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
