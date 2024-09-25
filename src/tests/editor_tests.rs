use crate::editor::texteditor::Textedit;
#[test]
fn test_read() {
    let mut textedit = Textedit::new();
    textedit.file = "src/tests/sample_inputs.txt".to_owned();
    textedit.read().unwrap();
    let contents = String::from_utf8(textedit.buffer)
        .unwrap_or_else(
            |_| panic!("Error reading file {}", textedit.file)
        );
    assert_eq!(contents,"int main() {\r\n    return 0;\r\n}");
    // \r is a carriage return, on windows it is needed in combination with \n.
    // if we want this to be cross-platform we'll have to find a way to handle this
    // windows : \r\n
    // mac     : \r or \n
    // linux   : \n
}

#[test]
fn test_write() {
    let mut textedit = Textedit::new();
    textedit.file = "src/tests/sample_inputs.txt".to_owned();
    textedit.read().unwrap();
    textedit.file = "src/tests/sample_outputs.txt".to_owned();
    textedit.write().unwrap();
    textedit.read().unwrap();
    let contents = String::from_utf8(textedit.buffer)
        .unwrap_or_else(
            |_| panic!("Error reading file {}", textedit.file)
        );
    assert_eq!(contents,"int main() {\r\n    return 0;\r\n}");
}