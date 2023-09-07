mod header;
mod message;
mod message_buffer;
mod question;
mod enums;
mod resource_record;

use std::io::Result;
use std::net::UdpSocket;
use crate::message::Message;
use crate::message_buffer::MessageBuffer;
use crate::header::Header;
use crate::question::Question;

/* TODO:
 * refactor
 * create recursive resolver,
 *   - bug where read_domain_name contains a pointer
 *   - given a domain name, get the IPv4 address that contains the rr for the domain
 *   - probably will print a graph of recursive queries done
 */
fn main() -> Result<()> {
    let mut header = Header::default();
    header.recursion_desired = true;
    header.qdcount = 1;
    header.id = 1997;
    let mut question = Question::default();
    question.qname = String::from("google.com");

    let mut packet = header.to_bytes();
    packet.append(&mut question.to_bytes());

    let result = UdpSocket::bind(("0.0.0.0", 8008));

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Ok(());
    }

    let socket = result.unwrap();

    let mut header = Header::default();

    while header.ancount == 0 {
        let result = socket.send_to(packet.as_slice(), ("198.41.0.4", 53)); //a.root-servers.net

        if result.is_err() {
            println!("{}", result.unwrap_err());
            return Ok(());
        }

        let mut buffer: [u8; 512] = [0; 512];
        socket.recv_from(&mut buffer)?;
        let message_buffer = MessageBuffer::new(buffer);
        let message = Message::from(message_buffer);

        println!("{:?}", message);

        header.ancount = 1;
    }

    return Ok(());
}
