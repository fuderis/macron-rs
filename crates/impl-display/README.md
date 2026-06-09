[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/impl-display)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-display)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-display)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl Display Macro

The implementation of trait [Display](std::fmt::Display).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).


## Examples:

### Struct
```rust
#[derive(Display)]
struct Test;
assert_eq!(Test {}.to_string(), "Test");

#[derive(Display)]
#[display(fmt = "Hello, {name}!")]
struct Hello {
    pub name: &'static str,
    pub age: u8,
}

assert_eq!(
    Hello {
        name: "User",
        age: 24
    }
    .to_string(),
    "Hello, User!"
);
```

### Enum
```rust
#[derive(Display)]
#[display(rename = "lowercase")]
enum Animals {
    Dog,

    Cat(&'static str'),

    #[display(fmt = "It's {0} tiger")]
    Tiger(&'static str, u8),

    #[display(fmt = "It's {name} bird")]
    Bird {
        name: &'static str,
        age: u8,
    },
}

assert_eq!(Animals::Dog.to_string(), "dog");
assert_eq!(Animals::Cat("Tom").to_string(), "Tom");
assert_eq!(Animals::Tiger("Simba", 1).to_string(), "It's Simba tiger");
assert_eq!(
    Animals::Bird {
        name: "Kesha",
        age: 1,
    }
    .to_string(),
    "It's Kesha bird"
);
```

## License & Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
