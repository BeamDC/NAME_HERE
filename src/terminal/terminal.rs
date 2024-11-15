use crate::editor::texteditor::Textedit;
use crate::traits::context::Context;

#[derive(Clone)]
pub struct Terminal {
    pub textedit: Textedit,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            textedit: Textedit::new(),
        }
    }
}

impl AsRef<Terminal> for Terminal {
    fn as_ref(&self) -> &Terminal {
        self
    }
}

impl Context for Terminal {
    fn name(&self) -> &'static str { "Terminal" }
    fn get_pointer(&self) -> usize { self.textedit.pointer}
    fn set_pointer(&mut self, pos: usize) { self.textedit.pointer = pos;}
    fn get_buffer(&self) -> &Vec<u8> { &self.textedit.buffer }
    fn get_buffer_mut(&mut self) -> &mut Vec<u8> { &mut self.textedit.buffer }
}
