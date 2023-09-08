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

    pub fn next_u16(&mut self) -> Option<u16> {
        let value1 = self.next();
        let value2 = self.next();

        if value1.is_none() || value2.is_none() {
            None
        } else {
            let mut res: u16 = 0;
            res |= (value1.unwrap() as u16) << 8;
            res |= value2.unwrap() as u16;
            Some(res)
        }
    }

    pub fn next_u32(&mut self) -> Option<u32> {
        let value1 = self.next_u16();
        let value2 = self.next_u16();

        if value1.is_none() || value2.is_none() {
            None
        } else {
            let mut res: u32 = 0;
            res |= (value1.unwrap() as u32) << 16;
            res |= value2.unwrap() as u32;
            Some(res)
        }
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
