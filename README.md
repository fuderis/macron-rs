[![github]](https://github.com/fuderis/macron-rs)&ensp;
[![crates-io]](https://crates.io/crates/macron)&ensp;
[![docs-rs]](https://docs.rs/macron)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Macros Toolkit

Comprehensive Rust macros toolkit for everyday development tasks. Includes convenient string formatting (str!), regex pattern matching (re!), streamlined collection creation, and custom derive macros for Display, Error, From, and Into traits. Boost your productivity with intuitive syntax and simplify your codebase while maintaining performance and safety. Perfect for both small projects and large-scale applications.

Empower your Rust development with this versatile set of macros, designed to make common tasks easier and more enjoyable.

## Navigation

#### File path (feature '_path_')

* __[macron-path](https://docs.rs/macron-path)__ - Creates a new instance of [PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html)

#### String operations (feature '_string_')

* __[macron-str](https://docs.rs/macron-str)__ - Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html)
* __[macron-regex](https://docs.rs/macron-regex)__ - Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)

#### IO Input (feature '_input_')

* __[macron-input](https://docs.rs/macron-input)__ - Reads user input lines from the console
* __[macron-inputln](https://docs.rs/macron-inputln)__ - Reads user input line from the console

#### Collections (feature '_collections_')

* __[macron-collections](https://docs.rs/macron-collections)__ - Creates a new instance of std collections: HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList and BinaryHeap

#### Derive macros (feature '_derive_')

* __[macron-impl-display](https://docs.rs/macron-impl-display)__ - The implementation of trait [Display](std::fmt::Display)
* __[macron-impl-error](https://docs.rs/macron-impl-error)__ - The implementation of trait [Error](std::error::Error)
* __[macron-impl-from](https://docs.rs/macron-impl-from)__ - The implementation of trait [From](std::convert::From)
* __[macron-impl-into](https://docs.rs/macron-impl-into)__ - The implementation of trait [Into](std::convert::Into)

### Note

If you need all of this macros, then use feature '_full_'.

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
