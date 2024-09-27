use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Textedit {
    pub file: String,
    pub buffer: Vec<u8>, // ascii 0 - 255 only :>
    pub pointer: (usize, usize),
}

impl Textedit {
    pub fn new() -> Self {
        Self {
            file: "src/editor/default.txt".to_owned(),
            buffer: vec![],
            pointer: (0, 0),
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
        f.write_all(contents)?;
        Ok(())
    }
}