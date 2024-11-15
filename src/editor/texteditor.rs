use std::fs;
use std::fs::File;
use std::io::Write;
use crate::traits::context::Context;

#[derive(Debug,Clone)]
pub struct Textedit {
    pub file: String,
    pub buffer: Vec<u8>, // ascii 0 - 255 only :>
    pub pointer: usize,
}

impl Textedit {
    pub fn new() -> Self {
        Self {
            file: "src/editor/default.txt".to_owned(),
            buffer: vec![],
            pointer: 0,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        self.buffer = Vec::new();
        self.buffer = fs::read(&self.file)?;
        Ok(())
    }

    pub fn write(&mut self) -> std::io::Result<()> {
        let mut f = File::create(&self.file)?;
        let contents: &[u8] = &self.buffer;
        let stripped = if contents[contents.len() - 1] == b'\0' {
            &contents[0..contents.len() - 1]
        }else{
            contents
        };
        f.write_all(stripped)?;
        Ok(())
    }
}

impl Context for Textedit {
    fn name(&self) -> &'static str { "Textedit" }
}