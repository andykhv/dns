use std::io;
use std::io::Read;
use std::fs::File;

use message_buffer::MessageBuffer;
use header::{Header, RCode, OpCode};

mod message_buffer;
mod header;

fn main() -> io::Result<()> {
    let mut message = MessageBuffer::new();
    let mut f = File::open("query_packet")?;
    let _ = f.read(&mut message.buffer);
    let header = Header::from(&message);

    println!("{:?}", header);

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

struct Question {
    qname: u64, //domain name
    qtype: u16, //type of query
    qclass: u16 //class of query
}

struct ResourceRecord {
    name: u64,      //domain name
    rtype: u16,     //type code of rdata
    class: u16,     //class of rdata
    ttl: u32,       //time interval (seconds) until cache -> trash
    rdlength: u16,  //length of rdata
    rdata: u16      //describes the resource
}
