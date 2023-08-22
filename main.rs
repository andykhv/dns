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

/*
* Header
* Question
* Answer (resource record)
* Authority (resource record)
* Additional (resource record)
* */
struct Message {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>
} 

struct ResourceRecord {
    name: u64,      //domain name
    rtype: u16,     //type code of rdata
    class: u16,     //class of rdata
    ttl: u32,       //time interval (seconds) until cache -> trash
    rdlength: u16,  //length of rdata
    rdata: u16      //describes the resource
}
