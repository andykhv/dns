use message_buffer::MessageBuffer;
use enums::Type;
use enums::Class;

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,   //domain name
    pub rtype: Type,    //type code of rdata
    pub rclass: Class,   //class of rdata
    pub ttl: u32,       //time interval (seconds) until cache -> trash
    pub rdlength: u16,  //length of rdata
    pub rdata: String   //describes the resource
}

impl ResourceRecord {
    fn new() -> ResourceRecord {
        ResourceRecord {
            name: String::from(""),
            rtype: Type::A,
            rclass: Class::IN,
            ttl: 0,
            rdlength: 0,
            rdata: String::from("")
        }
    }
}

impl From<&mut MessageBuffer> for ResourceRecord {
    fn from(message: &mut MessageBuffer) ->  ResourceRecord {
        let mut resource_record = ResourceRecord::new();

        let compression_mask = 0b1100_0000;
        let mut byte = message.next().unwrap_or_default();
        if byte == compression_mask {
            let pointer_mask = 0b0011_1111_1111_1111;
            let mut pointer: u16 = 0;
            pointer += byte as u16;
            pointer <<= 8;
            byte = message.next().unwrap_or_default();
            pointer |= byte as u16;
            pointer &= pointer_mask;

            let mut name = String::from("");
            let mut pointer = pointer as usize; //this can panic since pointer is originally u16

            while message.buffer[pointer] != 0 {
                let qname_count = message.buffer[pointer];

                for offset in 1..=qname_count {
                    let index = pointer + offset as usize;
                    let character = message.buffer[index] as char;
                    resource_record.name.push(character);
                }

                pointer += (qname_count as usize) + 1;

                //we reach the end if the current byte is 0
                if message.buffer[pointer] != 0 {
                    resource_record.name.push('.');
                }
            }
        }

        let mut type_value: u16 = 0;
        type_value += message.next().unwrap_or_default() as u16;
        type_value <<= 8;
        type_value |= message.next().unwrap_or_default() as u16;
        resource_record.rtype = Type::from(type_value);

        let mut class_value: u16 = 0;
        class_value += message.next().unwrap_or_default() as u16;
        class_value <<= 8;
        class_value |= message.next().unwrap_or_default() as u16;
        resource_record.rclass = Class::from(class_value);

        resource_record.ttl |= message.next().unwrap_or_default() as u32;
        resource_record.ttl <<= 24;
        resource_record.ttl |= message.next().unwrap_or_default() as u32;
        resource_record.ttl <<= 16;
        resource_record.ttl |= message.next().unwrap_or_default() as u32;
        resource_record.ttl <<= 8;
        resource_record.ttl |= message.next().unwrap_or_default() as u32;

        resource_record.rdlength |= message.next().unwrap_or_default() as u16;
        resource_record.rdlength <<= 8;
        resource_record.rdlength |= message.next().unwrap_or_default() as u16;

        for _ in 0..resource_record.rdlength {
            let value = message.next().unwrap_or_default();
            resource_record.rdata.push_str(value.to_string().as_str());
            resource_record.rdata.push('.');
        }

        resource_record.rdata.pop();

        return resource_record;
    }
}
