// MessageBuffer represents a raw 512 byte DNS message.
pub struct MessageBuffer {
    buffer: [u8; 512],
    position: usize
}

impl MessageBuffer {
    pub fn new(buffer: [u8; 512]) -> Self {
        MessageBuffer {
            buffer: buffer, position: 0
        }
    }

    pub fn seek(&mut self, position: usize) -> Result<(), &'static str>{
        if position >= self.buffer.len() {
            return Err("position out of bounds");
        }

        self.position = position;

        Ok(())
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
}

impl Iterator for MessageBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.position;
        self.position += 1;

        if pos < 512 {
            Some(self.buffer[pos])
        } else {
            None
        }
    }
}
