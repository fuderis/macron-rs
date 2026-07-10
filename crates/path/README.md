[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/path)&ensp;
[![crates-io]](https://crates.io/crates/macron-path)&ensp;
[![docs-rs]](https://docs.rs/macron-path)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# PathBuf Macro

A macro for ergonomic and safe construction of file and directory paths in Rust, with context-aware resolution.

* Supports both string literals and formatted paths with arguments.
* Automatically determines whether to allow a path relative to the current working directory, the executable directory, or the user data directory.

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).


## Path Patterns

The `path!` macro provides an intuitive template engine to resolve system-standard cross-platform directories at runtime without adding heavy external dependencies.

### Application Name Resolution

When appending a trailing dollar sign (`$`) to a token (e.g., `$config$`), the macro isolates data within an application-specific subdirectory.
* It checks for an ambient `APP_NAME` constant in the calling context.
* If `APP_NAME` is not found, it seamlessly falls back to the `CARGO_PKG_NAME` defined at compile time.

### 1. Executable & Environment Layouts

Dynamic layouts tied directly to the location of the running binary.

| Pattern   | Description                              | Linux / macOS / Windows Target                                 |
| :---      | :---                                     | :---                                                           |
| `$`       | Exact path to the current running binary | `/path/to/bin/executable`                                      |
| `$/`      | The directory containing the binary      | `/path/to/bin/`                                                |
| `$global` | System global application home folder    | `/opt` / `/Applications` / `C:\Program Files`                  |
| `$local`  | System local application home folder     | `~/.local/opt` / `~/Applications` / `%LOCALAPPDATA%\Programs`  |

### 2. System & Configuration Layouts (Isolated Contexts)

These targets are used for internal configuration, cache, and application runtimes. They utilize hidden folders on Unix systems or specific system locations on Windows.

| Pattern      | Linux / Unix Target                  | macOS Target                    | Windows Target        |
| :---         | :---                                 | :---                            | :---                  |
| `~`, `$home` | `/home/user`                         | `/Users/user`                   | `C:\Users\user`       |
| `$config`    | `~/.config` (`XDG_CONFIG_HOME`)      | `~/Library/Application Support` | `%APPDATA%`           |
| `$share`     | `~/.local/share` (`XDG_DATA_HOME`)   | `~/Library/Application Support` | `%LOCALAPPDATA%`      |
| `$cache`     | `~/.cache` (`XDG_CACHE_HOME`)        | `~/Library/Caches`              | `%LOCALAPPDATA%`      |
| `$temp`      | `/tmp`                               | `/tmp`                          | `%LOCALAPPDATA%\Temp` |

### 3. User Directories (Open Contexts)

Standard destination folders accessible directly by the user. When application routing is applied (`$token$`), directories are intentionally kept **unhidden** (without a leading dot) across all platforms for visibility.

| Pattern      | Target Subdirectory Name        | Linux / macOS / Windows Target        |
| :---         | :---                            | :---                                  |
| `$downloads` | `Downloads`                     | `~/Downloads/`                        |
| `$documents` | `Documents`                     | `~/Documents/`                        |
| `$music`     | `Music`                         | `~/Music/`                            |
| `$pictures`  | `Pictures`                      | `~/Pictures/`                         |
| `$videos`    | `Videos` (or `Movies` on macOS) | `~/Videos/` (or `~/Movies/` on macOS) |

## Examples

```rust
use macron_path::path;
use std::path::PathBuf;

// The application name is automatically integrated via your crate's APP_NAME constant
pub const APP_NAME: &str = "ovsy";

fn main() {
    // =========================================================================
    // 1. STATIC PATHS & TOKENS (Zero-Allocation for basic prefixes)
    // =========================================================================

    // Quick access to the current user's home directory
    let home_dir = path!("~");
    
    // Executable-relative path layout (ideal for self-contained deployments)
    let config_file = path!("$/config.toml");
    let asset_pack  = path!("$/assets/textures/skybox.png");


    // =========================================================================
    // 2. SYSTEM LAYOUTS & APPLICATION ISOLATION
    // =========================================================================
    // Prefix anchor ($) resolves to the global system directory.
    // Suffix anchor ($) isolates the path inside an app-specific subdirectory.

    // Global config directory:
    // Linux:   ~/.config/rules.ev
    // Windows: %APPDATA%\rules.ev
    let global_config = path!("$config/rules.ev");

    // Isolated app config directory (via trailing '$'):
    // Linux:   ~/.config/ovsy/rules.ev
    // Windows: %APPDATA%\ovsy\config\rules.ev
    let app_config = path!("$config$/rules.ev");
    
    // Isolated application cache
    // Linux:   ~/.cache/ovsy/storage.idx
    let cache_index = path!("$cache$/storage.idx");


    // =========================================================================
    // 3. USER-SPACE DIRECTORIES (Standard OS folders)
    // =========================================================================
    
    // User Documents directory with subfolders
    let export_dir = path!("$documents$/exports/report_2026.pdf");
    
    // Standard User Downloads folder
    let incoming   = path!("$downloads$/incoming/");


    // =========================================================================
    // 4. INLINED FORMATTING (Drop-in replacement for std::format!)
    // =========================================================================
    let log_file = "auth_service";
    let shard_id = 42;

    // Compiles using standard macro syntax, but returns a ready PathBuf
    let session_log = path!("$cache$/logs/{log_file}_shard_{shard_id}.log");
    let dynamic_img = path!("$pictures$/vacation/photo_{}.jpg", shard_id);


    // =========================================================================
    // 5. RUNTIME & DYNAMIC VARIABLES (String, &str, and PathBuf support)
    // =========================================================================
    
    // Passing a dynamic runtime String directly
    let runtime_string = String::from("$temp$/session.lock");
    let active_lock    = path!(runtime_string);

    // Safely forwarding an existing PathBuf containing internal tokens
    let raw_path    = PathBuf::from("$share$/production.db");
    let database_p  = path!(raw_path.to_string_lossy());
}
```

## License & Feedback

> This library distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
