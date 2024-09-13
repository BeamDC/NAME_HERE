use crate::editor::texteditor::Textedit;
#[test]
fn test_read() {
    let mut textedit = Textedit::new();
    textedit.file = "src/tests/sample_inputs.txt".to_owned();
    textedit.read().unwrap();
    assert_eq!(textedit.buffer
                   .iter()
                   .take_while(|&&c| c != '\0')
                   .copied()
                   .collect::<String>(),
               "int main() {\n    return 0;\n}\n");
}