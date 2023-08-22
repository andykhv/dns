#[derive(Debug)]
pub struct ResourceRecord {
    pub name: u64,      //domain name
    pub rtype: u16,     //type code of rdata
    pub class: u16,     //class of rdata
    pub ttl: u32,       //time interval (seconds) until cache -> trash
    pub rdlength: u16,  //length of rdata
    pub rdata: u16      //describes the resource
}


impl From<&MessageBuffer> for ResourceRecord {
    fn from(message: &MessageBuffer) -> ResourceRecord {
    }
}
