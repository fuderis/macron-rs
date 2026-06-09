extern crate macron_inputln;  use macron_inputln::inputln;

#[test]
fn test_inputln() {
    let input = inputln!("Enter word 'Test': ");
    assert_eq!(input, "Test");
}
