use message_buffer::MessageBuffer;
use enums::Type;
use enums::Class;

#[derive(Debug)]
pub struct Question {
    pub qname: String, //domain name
    pub qtype: Type, //type of query
    pub qclass: Class //class of query
}

impl Question {
    pub fn new() -> Self {
        Question {
            qname: String::from(""),
            qtype: Type::A,
            qclass: Class::IN
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
            1 => question.qtype = Type::A,
            2 => question.qtype = Type::NS,
            3 => question.qtype = Type::MD,
            4 => question.qtype = Type::MF,
            5 => question.qtype = Type::CNAME,
            _ => question.qtype = Type::NULL
        }

        current_index += 1;
        let mut qclass: u16 = message.buffer[current_index] as u16;
        qclass <<= 8;
        current_index += 1;
        qclass |= message.buffer[current_index] as u16;

        match qclass {
            1 => question.qclass = Class::IN,
            2 => question.qclass = Class::CS,
            3 => question.qclass = Class::CH,
            4 => question.qclass = Class::HS,
            _ => question.qclass = Class::IN
        }

        return question;
    }
}
