use message_buffer::MessageBuffer;

#[derive(Debug)]
pub struct Question {
    pub qname: String, //domain name
    pub qtype: QType, //type of query
    pub qclass: QClass //class of query
}

#[derive(Debug)]
pub enum QType {
    A       = 1,
    NS      = 2,
    MD      = 3,
    MF      = 4,
    CNAME   = 5,
    SOA     = 6,
    MB      = 7,
    MG      = 8,
    MR      = 9,
    NULL    = 10,
    WKS     = 11,
    PTR     = 12,
    HINFO   = 13,
    MINFO   = 14,
    MX      = 15,
    TXT     = 16
}

#[derive(Debug)]
pub enum QClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl Question {
    pub fn new() -> Self {
        Question {
            qname: String::from(""),
            qtype: QType::A,
            qclass: QClass::IN
        }
    }
}

impl From<&MessageBuffer> for Question {
    fn from(message: &MessageBuffer) -> Question {
        let mut question = Question::new();
        let mut current_index = 12;

        while message.buffer[current_index] != 0 {
            let qname_count = message.buffer[current_index];

            for byte in 1..=qname_count {
                let index = current_index + byte as usize;
                let character = message.buffer[index] as char;
                question.qname.push(character);
            }

            current_index += qname_count as usize;
            current_index += 1;

            if message.buffer[current_index] != 0 {
                question.qname.push('.');
            }
        }

        current_index += 1;

        let mut qtype: u16 = message.buffer[current_index] as u16;
        qtype <<= 8;
        current_index += 1;
        qtype |= message.buffer[current_index] as u16;

        match qtype {
            1 => question.qtype = QType::A,
            2 => question.qtype = QType::NS,
            3 => question.qtype = QType::MD,
            4 => question.qtype = QType::MF,
            5 => question.qtype = QType::CNAME,
            _ => question.qtype = QType::NULL
        }

        current_index += 1;
        let mut qclass: u16 = message.buffer[current_index] as u16;
        qclass <<= 8;
        current_index += 1;
        qclass |= message.buffer[current_index] as u16;

        match qclass {
            1 => question.qclass = QClass::IN,
            2 => question.qclass = QClass::CS,
            3 => question.qclass = QClass::CH,
            4 => question.qclass = QClass::HS,
            _ => question.qclass = QClass::IN
        }

        return question;
    }
}
