mod header;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io;
use std::io::Read;
use std::fs::File;
use message_buffer::MessageBuffer;
use header::{Header, RCode, OpCode};
use question::Question;
use resource_record::ResourceRecord;

/* TODO:
* use iterator for implementation
*/
fn main() -> io::Result<()> {
    let mut message = MessageBuffer::new();
    let mut f = File::open("response_packet")?;
    let _ = f.read(&mut message.buffer);
    let header = Header::from(&mut message);
    let question = Question::from(&message);
    let answer = ResourceRecord::from(&message);

    println!("{:?}", header);
    println!("{:?}", question);
    println!("{:?}", answer);

    Ok(())
}

struct Message {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>
} 


