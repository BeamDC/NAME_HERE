use crate::editor::texteditor::Textedit;
#[test]
fn test_textedit() {
    let mut textedit = Textedit::new();
    textedit.file = "src/tests/sample_inputs.txt".to_owned();
    textedit.read().unwrap();
    assert_eq!(textedit.buffer[0..=11].iter().copied().collect::<String>(), "Hello World!");
    // assert_eq!(textedit.buffer[0], 'H');
    // assert_eq!(textedit.buffer[1], 'e');
    // assert_eq!(textedit.buffer[2], 'l');
    // assert_eq!(textedit.buffer[3], 'l');
    // assert_eq!(textedit.buffer[4], 'o');
    // assert_eq!(textedit.buffer[5], ' ');
    // assert_eq!(textedit.buffer[6], 'W');
    // assert_eq!(textedit.buffer[7], 'o');
    // assert_eq!(textedit.buffer[8], 'r');
    // assert_eq!(textedit.buffer[9], 'l');
    // assert_eq!(textedit.buffer[10], 'd');
    // assert_eq!(textedit.buffer[11], '!');
}