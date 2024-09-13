use std::fs::File;
use std::io::BufRead;

pub struct Textedit {
    pub file: String,
    pub buffer: [char; 8192], // 8KB buffer, also happens to be the BufReader default size
    pub pointer: usize
}

impl Textedit {
    pub fn new() -> Self {
        Self {
            file: "default.txt".to_owned(), // I don't like the way this is done, looks bad
            buffer: ['\0'; 8192],
            pointer: 0,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        let f = File::open(&self.file)?;
        let reader = std::io::BufReader::new(f);

        for line in reader.lines() {
            let line = line?;
            for (i,c) in line.chars().enumerate() {
                self.buffer[i] = c;
            }
        }

        Ok(())
    }
}