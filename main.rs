use std::io;
use std::io::Read;
use std::fs::File;

mod message_buffer;

fn main() -> io::Result<()> {
    let mut message = message_buffer::MessageBuffer::new();
    let mut f = File::open("query_packet")?;
    let _ = f.read(&mut message.buffer);
    let mut header = Header::new();

    //id
    header.id += message.buffer[0] as u16;
    header.id <<= 8;
    header.id |= message.buffer[1] as u16;

    //qr
    let mut mask: u8 = 0b1000_0000;
    header.qr = message.buffer[2] & mask == mask;

    //opcode
    mask = 0b0111_1000;
    println!("{}", mask);
    println!("{}", message.buffer[2]);
    let mut opcode = message.buffer[2] & mask;
    opcode >>= 3;
    opcode += 0b0000_0001;
    match opcode {
        0 =>  header.opcode = OpCode::QUERY,
        1 =>  header.opcode = OpCode::IQUERY,
        2 =>  header.opcode = OpCode::STATUS,
        3 =>  header.opcode = OpCode::OTHER,
        _ =>  header.opcode = OpCode::QUERY,
    }

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

struct Header {
    id: u16,
    qr: bool,       //0 = query, 1 = response
    opcode: OpCode,
    aa: bool,       //authoratative answer
    tc: bool,       //truncated
    rd: bool,       //recursion desired
    ra: bool,       //recusion available
    rcode: RCode,
    qdcount: u16,   //# of entries in question section
    ancount: u16,   //# of resource records in answer section
    nscount: u16,   //# of name server resource records in authority records section
    arcount: u16,   //# of records in additional resource records section
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: 0,
            qr: false,
            opcode: OpCode::QUERY,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            rcode: RCode::NoError,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0
        }
    }
}

enum OpCode {
    QUERY  = 0,
    IQUERY = 1,
    STATUS = 2,
    OTHER  = 3
}

enum RCode {
    NoError         = 0,
    FormatError     = 1,
    ServerFailure   = 2,
    NameError       = 3,
    NotImplemented  = 4,
    Refused         = 5
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
