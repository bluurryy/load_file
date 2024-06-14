use load_file::load_str;

#[test]
fn integration_test() {
    let content = load_str!("greeting.txt");
    assert_eq!(content, "Hello integration test in workspace!\n");
}
