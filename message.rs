use header::Header;
use question::Question;
use resource_record::ResourceRecord;

#[derive(Debug, Default)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<ResourceRecord>
}

