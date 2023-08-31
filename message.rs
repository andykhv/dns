use crate::header::Header;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

#[derive(Debug, Default)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<ResourceRecord>
}

