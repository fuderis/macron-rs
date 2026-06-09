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
    #[display(rename = "lowercase")]
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
