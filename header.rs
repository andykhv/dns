use crate::message_buffer::MessageBuffer;

#[derive(Debug, Default)]
pub struct Header {
    pub id: u16,
    pub qr: bool,
    pub opcode: OpCode,
    pub authoritative_answer: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub rcode: RCode,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

#[derive(Debug, Default)]
pub enum OpCode {
    #[default]
    QUERY  = 0,
    IQUERY = 1,
    STATUS = 2,
    OTHER  = 3
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

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> u8 {
        match value {
            OpCode::QUERY  => 0b0000_0000,
            OpCode::IQUERY => 0b0000_0001,
            OpCode::STATUS => 0b0000_0010,
            OpCode::OTHER  => 0b0000_0011,
        }
    }
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

impl From<RCode> for u8 {
    fn from(value: RCode) -> u8 {
        match value {
            RCode::NoError        => 0b0000_0000,
            RCode::FormatError    => 0b0000_0001,
            RCode::ServerFailure  => 0b0000_0010,
            RCode::NameError      => 0b0000_0011,
            RCode::NotImplemented => 0b0000_0100,
            RCode::Refused        => 0b0000_0101,
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
        header.authoritative_answer = byte & mask == mask;

        //tc
        mask = 0b0000_0010;
        header.truncated = byte & mask == mask;

        //rd
        mask = 0b0000_0001;
        header.recursion_desired = byte & mask == mask;

        byte = message.next().unwrap_or_default();
        //ra
        mask = 0b1000_0000;
        header.recursion_available = byte & mask == mask;

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

impl Header {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let id = self.id.to_be_bytes();
        bytes.push(id[0]);
        bytes.push(id[1]);
        
        let mut byte: u8 = 0;
        let qr = u8::from(self.qr) << 7;
        byte |= qr;

        let opcode = (u8::from(self.opcode)) << 6;
        byte |= opcode;

        let aa = u8::from(self.authoritative_answer) << 2;
        byte |= aa;

        let tc = u8::from(self.truncated) << 1;
        byte |= tc;

        let rd = u8::from(self.recursion_desired);
        byte |= rd;

        bytes.push(byte);

        byte = 0;

        let ra = (u8::from(self.recursion_available)) << 7;
        byte |= ra;

        let rcode = u8::from(self.rcode);
        byte |= rcode;

        bytes.push(byte);

        let qdcount = self.qdcount.to_be_bytes();
        bytes.push(qdcount[0]);
        bytes.push(qdcount[1]);

        let ancount = self.ancount.to_be_bytes();
        bytes.push(ancount[0]);
        bytes.push(ancount[1]);

        let nscount = self.nscount.to_be_bytes();
        bytes.push(nscount[0]);
        bytes.push(nscount[1]);

        let arcount = self.arcount.to_be_bytes();
        bytes.push(arcount[0]);
        bytes.push(arcount[1]);

        return bytes;
    }
}
