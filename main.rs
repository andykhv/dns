mod header;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io;
use std::io::Read;
use std::fs::File;
use message_buffer::MessageBuffer;
use header::Header;
use question::Question;
use resource_record::ResourceRecord;

fn main() -> io::Result<()> {
    let mut message_buffer = MessageBuffer::new();
    let mut f = File::open("query_packet")?;
    let _ = f.read(&mut message_buffer.buffer);
    
    let mut message = Message::new();
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

#[derive(Debug)]
struct Message {
    header: Header,
    question: Vec<Question>,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>
}

impl Message {
    fn new() -> Self {
        Message {
            header: Header::new(),
            question: Vec::new(),
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        }
    }
}

