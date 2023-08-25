use message_buffer::MessageBuffer;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub qr: bool,       //0 = query, 1 = response
    pub opcode: OpCode,
    pub aa: bool,       //authoratative answer
    pub tc: bool,       //truncated
    pub rd: bool,       //recursion desired
    pub ra: bool,       //recursion available
    pub rcode: RCode,
    pub qdcount: u16,   //# of entries in question section
    pub ancount: u16,   //# of resource records in answer section
    pub nscount: u16,   //# of name server resource records in authority records section
    pub arcount: u16,   //# of records in additional resource records section
}

#[derive(Debug)]
pub enum OpCode {
    QUERY  = 0,
    IQUERY = 1,
    STATUS = 2,
    OTHER  = 3
}

#[derive(Debug)]
pub enum RCode {
    NoError         = 0,
    FormatError     = 1,
    ServerFailure   = 2,
    NameError       = 3,
    NotImplemented  = 4,
    Refused         = 5
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

impl From<&MessageBuffer> for Header {
    fn from(message: &MessageBuffer) -> Self {
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

        //nscount
        header.nscount += message.buffer[8] as u16;
        header.nscount <<= 8;
        header.nscount |= message.buffer[9] as u16;

        //qdcount
        header.arcount += message.buffer[10] as u16;
        header.arcount <<= 8;
        header.arcount |= message.buffer[11] as u16;

        return header;
    }
}


