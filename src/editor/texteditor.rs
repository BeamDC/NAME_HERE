use std::fs::File;
use std::io::BufRead;

const BUFFER_SIZE: usize = 8192;

pub struct Textedit {
    pub file: String,
    pub buffer: [char; BUFFER_SIZE], // 8KiB buffer, also happens to be the BufReader default size, change to make bigger
    pub pointer: usize
}

impl Textedit {
    pub fn new() -> Self {
        Self {
            file: "default.txt".to_owned(),
            buffer: ['\0'; BUFFER_SIZE],
            pointer: 0,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        // todo: if new file is selected, overwrite current buffer with nulls before reading

        let f = File::open(&self.file)?;
        let reader = std::io::BufReader::with_capacity(BUFFER_SIZE,f);

        for line in reader.lines() {
            let line = line?;
            for (i, c) in line.chars().enumerate() {
                if self.pointer + i < self.buffer.len() {
                    self.buffer[self.pointer + i] = c;
                } else {
                    break;
                }
            }
            self.pointer += line.chars().count();
            self.buffer[self.pointer] = '\n';
            self.pointer += 1;
        }
        self.pointer = 0;
        Ok(())
    }
}