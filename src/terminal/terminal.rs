use crate::traits::context::Context;

#[derive(Clone)]
pub struct Terminal {
    pub buffer: Vec<u8>,
    pub pointer: usize,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            pointer: 0,
        }
    }

    pub fn write() {
        // nothing
    }
}

impl Context for Terminal {
    fn name(&self) -> &'static str { "Terminal" }
}
