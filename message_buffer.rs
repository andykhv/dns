
/* MessageBuffer represents a 512 byte DNS message.
 * 
 */
struct MessageBuffer {
    buffer: [u8; 512],
    position: usize
}

impl MessageBuffer {
    fn new() -> MessageBuffer {
        MessageBuffer {
            buffer: [0; 512],
            position: 0
        }
    }
}

impl Iterator for MessageBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;

        if self.position < 512 {
            Some(self.buffer[self.position])
        } else {
            None
        }
    }
}
