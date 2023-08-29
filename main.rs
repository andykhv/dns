mod header;
mod message;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io::{Read, Result};
use std::fs::File;
use message::Message;
use message_buffer::MessageBuffer;
use header::Header;
use question::Question;
use resource_record::ResourceRecord;

fn main() -> Result<()> {
    let mut message_buffer = MessageBuffer::default();
    let mut f = File::open("query_packet")?;
    let _ = f.read(&mut message_buffer.buffer);
    
    let mut message = Message::default();
    message.header = Header::from(&mut message_buffer);

    for _ in 0..message.header.qdcount {
        message.question.push(Question::from(&mut message_buffer));
    }

    for _ in 0..message.header.ancount {
        message.answer.push(ResourceRecord::from(&mut message_buffer));
    }

    println!("{:?}", message);
    Ok(())
}

