mod header;
mod message;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io::{Result, Error, ErrorKind};
use std::net::UdpSocket;
use crate::enums::Type;
use crate::message::Message;
use crate::message_buffer::MessageBuffer;
use crate::header::Header;
use crate::question::Question;

fn main() -> Result<()> {
    let result = UdpSocket::bind(("127.0.0.1", 8100));

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Ok(());
    }

    let query_socket = result.unwrap();

    println!("{:?}", query_socket);

    loop {
        let mut query_buffer: [u8; 512] = [0; 512];
        let (_, src_address) = query_socket.recv_from(&mut query_buffer)?;
        let message_buffer = MessageBuffer::new(query_buffer);
        let query_message = Message::from(message_buffer);

        let mut message = resolve_question(&query_message.questions[0].qname, query_message.questions[0].qtype)?;
        message.header.id = query_message.header.id;

        let mut buffer = message.header.to_be_bytes();

        for i in 0..message.header.qdcount {
            let j = i as usize;
            println!("{}", j);
            buffer.append(&mut message.questions[j].to_be_bytes());
        }
        println!("{:?}", message.answers);
        for i in 0..message.header.ancount {
            let j = i as usize;
            println!("{}", j);
            buffer.append(&mut message.answers[j].to_be_bytes());
        }
        for i in 0..message.header.nscount {
            let j = i as usize;
            println!("{}", j);
            buffer.append(&mut message.authorities[j].to_be_bytes());
        }
        for i in 0..message.header.arcount {
            let j = i as usize;
            println!("{}", j);
            buffer.append(&mut message.additional[j].to_be_bytes());
        }

        query_socket.send_to(&buffer, src_address)?;
    }
}

fn resolve_question(qname: &str, qtype: Type) -> Result<Message> {
    let mut target = String::from("198.41.0.4");
    let target_port: u16 = 53;
    let socket = UdpSocket::bind(("127.0.0.1", 8000))?;

    let mut header = Header::default();
    header.id = 1997;
    header.qdcount = 1;
    header.recursion_desired = true;

    let mut question = Question::default();
    question.qname = String::from(qname);
    question.qtype = qtype;

    let mut send_buffer = header.to_be_bytes();
    send_buffer.append(&mut question.to_be_bytes());

    loop {
        let _ = socket.send_to(&send_buffer, (target.as_str(), target_port));
        
        let mut receive_buffer: [u8; 512] = [0; 512];
        let _ = socket.recv_from(&mut receive_buffer);

        let message_buffer = MessageBuffer::new(receive_buffer);
        let message = Message::from(message_buffer);

        if message.header.ancount != 0 {
            return Ok(message);
        }

        let authority = message.authorities.iter().next();

        match authority {
            Some(a) => target = a.rdata.to_owned(),
            None => return Err(Error::new(ErrorKind::NotFound, "question unresolvable: authority not found")) 
        }
    }
}
