use std::fmt;

pub struct GapBuffer {
    pub buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl fmt::Debug for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gap Start: {:?}\nGap End: {:?}\nBuffer: {:?}", self.gap_start, self.gap_end, self.buffer)
    }
}

impl GapBuffer {
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, '\0');

        Self {
            buffer,
            gap_start: 0,
            gap_end: size
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }

    // grow the buffer
    fn grow(&mut self) {
        // todo : potential edge case shen length is 0
        let old_size = self.buffer.len();
        let new_size = old_size * 2;
        let mut new_buffer = Vec::with_capacity(new_size);

        new_buffer.extend_from_slice(&self.buffer[0..self.gap_start]);

        let new_gap_size = new_size - old_size;
        new_buffer.resize(self.gap_start + new_gap_size, '\0');

        new_buffer.extend_from_slice(&self.buffer[self.gap_end..]);

        self.buffer = new_buffer;
        self.gap_end = self.gap_start + new_gap_size;
    }

    // move the gap to a specific position
    fn move_gap_to(&mut self, pos: usize) {
        if pos == self.gap_start {
            return;
        }

        if pos < self.gap_start {
            // Move gap left
            let distance = self.gap_start - pos;
            for i in 0..distance {
                self.buffer[self.gap_end - 1 - i] = self.buffer[self.gap_start - 1 - i];
            }
        } else {
            // Move gap right
            let distance = pos - self.gap_start;
            for i in 0..distance {
                self.buffer[self.gap_start + i] = self.buffer[self.gap_end + i];
            }
        }

        if pos < self.gap_start {
            self.gap_end -= self.gap_start - pos;
            self.gap_start = pos;
        } else {
            self.gap_start = pos;
            self.gap_end += pos - self.gap_start;
        }
    }

    // insert a character into the buffer
    pub fn insert(&mut self, pos: usize, ch: char) {
        if self.gap_end == self.gap_start {
            // Gap is empty, need to grow
            self.grow();
        }

        self.move_gap_to(pos);
        self.buffer[self.gap_start] = ch;
        self.gap_start += 1;
    }

    // insert a string into the buffer
    pub fn insert_str(&mut self, pos: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.insert(pos + i, ch);
        }
    }

    // delete an element from the buffer
    pub fn delete(&mut self, pos: usize) {
        if pos >= self.len() {
            return;
        }

        if pos != self.gap_start {
            self.move_gap_to(pos);
        }
        self.gap_end += 1;
    }

    // insert a string into the buffer for editing
    pub fn from_str(s: &str) -> Self {
        let capacity = s.len() * 2;
        let mut gb = Self::new(capacity);
        gb.insert_str(0, s);
        gb
    }

    // turn the buffer into a string for parsing
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.len());
        for i in 0..self.gap_start {
            result.push(self.buffer[i]);
        }
        for i in self.gap_end..self.buffer.len() {
            result.push(self.buffer[i]);
        }
        result
    }

    // todo : turn the buffer into a vec<u8>
}