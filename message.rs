use header::Header;
use question::Question;
use resource_record::ResourceRecord;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<ResourceRecord>
}

impl Message {
    pub fn new() -> Self {
        Message {
            header: Header::new(),
            question: Vec::new(),
            answer: Vec::new()
        }
    }
}
