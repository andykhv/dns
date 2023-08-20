
/* MessageBuffer represents a 512 byte DNS message.
 * 
 */
pub struct MessageBuffer {
    pub buffer: [u8; 512],
    pub position: usize
}

impl MessageBuffer {
    pub fn new() -> MessageBuffer {
        MessageBuffer {
            buffer: [0; 512],
            position: 0
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
