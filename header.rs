use message_buffer::MessageBuffer;

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub enum OpCode {
    #[default]
    QUERY  = 0,
    IQUERY = 1,
    STATUS = 2,
    OTHER  = 3
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::QUERY,
            1 => OpCode::IQUERY,
            2 => OpCode::STATUS,
            3 => OpCode::OTHER,
            _ => OpCode::QUERY,
        }
    }
}

#[derive(Debug, Default)]
pub enum RCode {
    #[default]
    NoError         = 0,
    FormatError     = 1,
    ServerFailure   = 2,
    NameError       = 3,
    NotImplemented  = 4,
    Refused         = 5
}

impl From<u8> for RCode {
    fn from(value: u8) -> Self {
        match value {
            0 => RCode::NoError,
            1 => RCode::FormatError,
            2 => RCode::ServerFailure,
            3 => RCode::NameError,
            4 => RCode::NotImplemented,
            5 => RCode::Refused,
            _ => RCode::NoError
        }
    }
}

impl From<&mut MessageBuffer> for Header {
    fn from(message: &mut MessageBuffer) -> Self {
        let mut header = Header::default();

        //id
        header.id += message.next().unwrap_or_default() as u16;
        header.id <<= 8;
        header.id |= message.next().unwrap_or_default() as u16;

        //byte will be masked to get the next fields
        let mut byte: u8 = message.next().unwrap_or_default();

        //qr
        let mut mask: u8 = 0b1000_0000;
        header.qr = byte & mask == mask;

        //opcode
        mask = 0b0111_1000;
        let mut opcode = byte & mask;
        opcode >>= 3;
        header.opcode = OpCode::from(opcode);

        //aa
        mask = 0b0000_0100;
        header.aa = byte & mask == mask;

        //tc
        mask = 0b0000_0010;
        header.tc = byte & mask == mask;

        //rd
        mask = 0b0000_0001;
        header.rd = byte & mask == mask;

        byte = message.next().unwrap_or_default();
        //ra
        mask = 0b1000_0000;
        header.ra = byte & mask == mask;

        //rcode
        mask = 0b0000_1111;
        let rcode = byte & mask;
        header.rcode = RCode::from(rcode);

        //qdcount
        header.qdcount += message.next().unwrap_or_default() as u16;
        header.qdcount <<= 8;
        header.qdcount |= message.next().unwrap_or_default() as u16;

        //ancount
        header.ancount += message.next().unwrap_or_default() as u16;
        header.ancount <<= 8;
        header.ancount |= message.next().unwrap_or_default() as u16;

        //nscount
        header.nscount += message.next().unwrap_or_default() as u16;
        header.nscount <<= 8;
        header.nscount |= message.next().unwrap_or_default() as u16;

        //qdcount
        header.arcount += message.next().unwrap_or_default() as u16;
        header.arcount <<= 8;
        header.arcount |= message.next().unwrap_or_default() as u16;

        return header;
    }
}


