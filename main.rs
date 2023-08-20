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

    //aa
    mask = 0b0000_0100;
    header.aa = message.buffer[2] & mask == mask;

    //tc
    mask = 0b0000_0010;
    header.tc = message.buffer[2] & mask == mask;

    //rd
    mask = 0b0000_0001;
    header.rd = message.buffer[2] & mask == mask;

    //ra
    mask = 0b1000_0000;
    header.ra = message.buffer[3] & mask == mask;

    //rcode
    mask = 0b0000_1111;
    let rcode = message.buffer[3] & mask;
    match rcode {
        0 => header.rcode = RCode::NoError,
        1 => header.rcode = RCode::FormatError,
        2 => header.rcode = RCode::ServerFailure,
        3 => header.rcode = RCode::NameError,
        4 => header.rcode = RCode::NotImplemented,
        5 => header.rcode = RCode::Refused,
        _ => header.rcode = RCode::NoError
    }

    //qdcount
    header.qdcount += message.buffer[4] as u16;
    header.qdcount <<= 8;
    header.qdcount |= message.buffer[5] as u16;

    //ancount
    header.ancount += message.buffer[6] as u16;
    header.ancount <<= 8;
    header.ancount |= message.buffer[7] as u16;

    //qdcount
    header.ancount += message.buffer[8] as u16;
    header.ancount <<= 8;
    header.ancount |= message.buffer[9] as u16;

    //qdcount
    header.ancount += message.buffer[10] as u16;
    header.ancount <<= 8;
    header.ancount |= message.buffer[11] as u16;

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
#[derive(Debug)]
struct Message {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>
} 

#[derive(Debug)]
struct Header {
    id: u16,
    qr: bool,       //0 = query, 1 = response
    opcode: OpCode,
    aa: bool,       //authoratative answer
    tc: bool,       //truncated
    rd: bool,       //recursion desired
    ra: bool,       //recursion available
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

#[derive(Debug)]
enum OpCode {
    QUERY  = 0,
    IQUERY = 1,
    STATUS = 2,
    OTHER  = 3
}

#[derive(Debug)]
enum RCode {
    NoError         = 0,
    FormatError     = 1,
    ServerFailure   = 2,
    NameError       = 3,
    NotImplemented  = 4,
    Refused         = 5
}

#[derive(Debug)]
struct Question {
    qname: u64, //domain name
    qtype: u16, //type of query
    qclass: u16 //class of query
}

#[derive(Debug)]
struct ResourceRecord {
    name: u64,      //domain name
    rtype: u16,     //type code of rdata
    class: u16,     //class of rdata
    ttl: u32,       //time interval (seconds) until cache -> trash
    rdlength: u16,  //length of rdata
    rdata: u16      //describes the resource
}
