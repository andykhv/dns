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

impl From<&mut MessageBuffer> for Question {
    fn from(message: &mut MessageBuffer) -> Question {
        let mut question = Question::new();
        let mut byte = message.next().unwrap_or_default();

        while byte != 0 {
            let qname_count = byte;

            for _ in 0..qname_count {
                let character = message.next().unwrap_or_default() as char;
                question.qname.push(character);
            }

            question.qname.push('.');
            byte = message.next().unwrap_or_default();
        }

        question.qname.pop();

        let mut qtype: u16 = message.next().unwrap_or_default() as u16;
        qtype <<= 8;
        qtype |= message.next().unwrap_or_default() as u16;
        question.qtype = Type::from(qtype);

        let mut qclass: u16 = message.next().unwrap_or_default() as u16;
        qclass <<= 8;
        qclass |= message.next().unwrap_or_default() as u16;
        question.qclass = Class::from(qclass);

        return question;
    }
}
