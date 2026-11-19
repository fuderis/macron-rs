#![allow(dead_code, unused_assignments, unused_variables)]
use macron_impl_display::Display;

#[test]
fn test_struct_display() {
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
}

#[test]
fn test_enum_display() {
    #[derive(Display)]
    #[display(rename_all = "lowercase")]
    enum Animals {
        Dog,

        Cat(&'static str),

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
}

#[test]
fn test_let_bindings_display() {
    #[derive(Display)]
    #[display(
        fmt = "{a} {b}",
        a = "bla-bla",
        b = opt.as_deref().unwrap_or("-")
    )]
    struct Item {
        opt: Option<String>,
    }

    assert_eq!(
        Item {
            opt: Some("ok".to_string())
        }
        .to_string(),
        "bla-bla ok"
    );
    assert_eq!(Item { opt: None }.to_string(), "bla-bla -");

    #[derive(Display)]
    enum LogEntry {
        #[display(
            fmt = "[{level}] {msg}",
            level = if *_1 { "WARN" } else { "INFO" },
            msg = _0.trim()
        )]
        Message(String, bool),

        #[display(
            fmt = "{0}",
            _0 = _0.to_uppercase()
        )]
        Code(String),

        #[display(
            fmt = "{user} ({status})",
            status = if *retries > 0 { "retry" } else { "fresh" }
        )]
        Job { user: &'static str, retries: u8 },
    }

    assert_eq!(
        LogEntry::Message("  hello  ".to_string(), false).to_string(),
        "[INFO] hello"
    );
    assert_eq!(
        LogEntry::Message("disk full".to_string(), true).to_string(),
        "[WARN] disk full"
    );
    assert_eq!(LogEntry::Code("err_404".to_string()).to_string(), "ERR_404");
    assert_eq!(
        LogEntry::Job {
            user: "alice",
            retries: 2,
        }
        .to_string(),
        "alice (retry)"
    );
}
