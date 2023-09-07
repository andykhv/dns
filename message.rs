use crate::header::Header;
use crate::message_buffer::MessageBuffer;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

#[derive(Debug, Default)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additional: Vec<ResourceRecord>
}

impl From<MessageBuffer> for Message {
    fn from(mut buffer: MessageBuffer) -> Self {
        let mut message = Message::default();
        message.header = Header::from(&mut buffer);

        for _ in 0..message.header.qdcount {
            message.questions.push(Question::from(&mut buffer));
        }

        for _ in 0..message.header.ancount {
            message.answers.push(ResourceRecord::from(&mut buffer));
        }

        for _ in 0..message.header.nscount {
            message.authorities.push(ResourceRecord::from(&mut buffer));
        }

        for _ in 0..message.header.arcount {
            message.additional.push(ResourceRecord::from(&mut buffer));
        }

        return message;
    }
}

