extern crate macron_input;  use macron_input::input;

#[test]
fn test_input() {
    let mut input = input!("Enter in order '0', '1', '2': ");

    for i in 0..=2 {
        if let Ok(text) = input.next().unwrap() {
            assert_eq!(text, format!("{i}"));
        }
    }
}
