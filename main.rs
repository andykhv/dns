mod header;
mod message_buffer;
mod question;

use std::io;
use std::io::Read;
use std::fs::File;
use message_buffer::MessageBuffer;
use header::{Header, RCode, OpCode};
use question::Question;

fn main() -> io::Result<()> {
    let mut message = MessageBuffer::new();
    let mut f = File::open("query_packet")?;
    let _ = f.read(&mut message.buffer);
    let header = Header::from(&message);
    let question = Question::from(&message);

    println!("{:?}", header);
    println!("{:?}", question);

    Ok(())
}

struct Message {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>
} 


