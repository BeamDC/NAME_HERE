use std::fs;
use std::fs::File;
use std::io::Write;
use crate::traits::context::{Context};

#[derive(Debug,Clone)]
pub struct Textedit {
    pub file: Option<String>,
    pub buffer: Vec<u8>, // ascii 0 - 255 only :>
    pub pointer: usize,
}

impl Textedit {
    pub fn new() -> Self {
        Self {
            file: None,
            buffer: vec![],
            pointer: 0,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        if self.file.is_none() { return Ok(())}
        let path = self.file.clone().unwrap();

        self.buffer = Vec::new();
        self.buffer = fs::read(&path)?;
        Ok(())
    }
    pub fn write(&mut self) -> std::io::Result<()> {
        if self.file.is_none() { return Ok(())}
        let path = self.file.clone().unwrap();

        let mut f = File::create(&path)?;
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
    fn name(&self) -> &'static str { "Editor" }
    fn get_pointer(&self) -> usize { self.pointer}
    fn set_pointer(&mut self, pos: usize) { self.pointer = pos;}
    fn get_buffer(&self) -> &Vec<u8> { &self.buffer }
    fn get_buffer_mut(&mut self) -> &mut Vec<u8> { &mut self.buffer }
}