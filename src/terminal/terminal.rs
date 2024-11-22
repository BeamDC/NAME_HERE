use crate::editor::texteditor::Textedit;
use crate::editor::context::Context;

// use this later for more advanced responses
#[derive(Clone, Debug)]
pub struct TerminalResponse {
    pub response: String,
}

#[derive(Clone)]
pub struct Terminal {
    pub textedit: Textedit,
    pub responses: Vec<TerminalResponse>,
    pub current_response: TerminalResponse,
}

impl TerminalResponse {
    pub fn new() -> Self {
        Self {
            response: String::new(),
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            textedit: Textedit::new(),
            responses: Vec::new(),
            current_response: TerminalResponse::new(),
        }
    }

    pub fn get_command(&self) -> String {
        let last_line = self.textedit.buffer
            .iter()
            .rposition(|&x| x == b'\n');
        let slice = match last_line {
            Some(pos) => &self.textedit.buffer[pos + 1..],
            None => &self.textedit.buffer,
        };
        String::from_utf8(slice.to_vec()).unwrap()
    }

    pub fn get_response(&mut self){
        let response_string = self.get_command();

        let response = TerminalResponse {
            response: response_string,
        };
        self.current_response = response;
    }

    pub fn add_response(&mut self){
        self.responses.push(self.current_response.clone());
    }
}

impl Context for Terminal {
    // fn name(&self) -> &'static str { "Terminal" }
    // fn get_pointer(&self) -> usize { self.textedit.pointer}
    // fn set_pointer(&mut self, pos: usize) { self.textedit.pointer = pos;}
    // fn get_buffer(&self) -> &Vec<u8> { &self.textedit.buffer }
    // fn get_buffer_mut(&mut self) -> &mut Vec<u8> { &mut self.textedit.buffer }
}
