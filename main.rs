mod header;
mod message;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io::{Read, Write, Result};
use std::fs::File;
use std::net::UdpSocket;
use crate::message::Message;
use crate::message_buffer::MessageBuffer;
use crate::header::Header;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

fn main() -> Result<()> {
    let mut message_buffer = MessageBuffer::default();
    let mut f = File::open("./packets/query_packet")?;
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

    let mut header = Header::default();
    header.rd = true;
    header.qdcount = 1;
    header.id = 192;
    let mut question = Question::default();
    question.qname = String::from("blog.andykhov.xyz");

    let mut packet = header.to_bytes();
    packet.append(&mut question.to_bytes());

    let result = UdpSocket::bind(("0.0.0.0", 8008));

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Ok(());
    }

    let socket = result.unwrap();
    let result = socket.send_to(packet.as_slice(), ("8.8.8.8", 53));

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Ok(());
    }

    let mut receive_buffer = MessageBuffer::default();
    socket.recv_from(&mut receive_buffer.buffer)?;

    let header = Header::from(&mut receive_buffer);
    let question = Question::from(&mut receive_buffer);
    let r1 = ResourceRecord::from(&mut receive_buffer);
    let r2 = ResourceRecord::from(&mut receive_buffer);

    println!("{:?}", header);
    println!("{:?}", question);
    println!("{:?}", r1);
    println!("{:?}", r2);

    return Ok(());
}

