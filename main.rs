mod message_buffer;

fn main() {

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
