use crate::editor::texteditor::Textedit;
use crate::traits::context::Context;

#[derive(Clone)]
pub struct Terminal {
    pub textedit: Textedit,
}

// todo: make this work.
// maybe double the line spacing, to have a line in between of terminal output.
// or write output to the buffer, and force the cursor to the end.
impl Terminal {
    pub fn new() -> Self {
        Self {
            textedit: Textedit::new(),
        }
    }
}

impl Context for Terminal {
    fn name(&self) -> &'static str { "Terminal" }
    fn get_pointer(&self) -> usize { self.textedit.pointer}
    fn set_pointer(&mut self, pos: usize) { self.textedit.pointer = pos;}
    fn get_buffer(&self) -> &Vec<u8> { &self.textedit.buffer }
    fn get_buffer_mut(&mut self) -> &mut Vec<u8> { &mut self.textedit.buffer }
}
