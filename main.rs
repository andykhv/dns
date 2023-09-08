mod header;
mod message;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io::Result;
use std::net::UdpSocket;
use crate::enums::Type;
use crate::message::Message;
use crate::message_buffer::MessageBuffer;
use crate::header::Header;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

/* TODO:
 * create UDP server
 * probably will print a graph of recursive queries done
 */
fn main() -> Result<()> {
    let target = "google.com";
    let mut host = String::from("198.41.0.4"); //a.root-servers.net
    let result = UdpSocket::bind(("0.0.0.0", 8008));

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Ok(());
    }

    let socket = result.unwrap();
    let mut message = Message::default();

    while message.header.ancount == 0 {
        let mut header = Header::default();
        header.recursion_desired = true;
        header.qdcount = 1;
        header.id = 1997;
        let mut question = Question::default();
        question.qname = String::from(target);

        let mut packet = header.to_bytes();
        packet.append(&mut question.to_bytes());

        let result = socket.send_to(packet.as_slice(), (host.as_str(), 53)); //a.root-servers.net

        if result.is_err() {
            println!("{}", result.unwrap_err());
            return Ok(());
        }

        let mut buffer: [u8; 512] = [0; 512];
        socket.recv_from(&mut buffer)?;
        let message_buffer = MessageBuffer::new(buffer);
        message = Message::from(message_buffer);

        if message.header.nscount > 0 {
            let addresses: Vec<&ResourceRecord> = message.authorities.iter().filter(|r| r.rtype == Type::NS).collect();
            host = addresses[0].rdata.clone();

            println!("{}", host);
        }
    }

    println!("{:?}", message);

    return Ok(());
}
