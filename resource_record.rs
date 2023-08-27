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

impl From<&MessageBuffer> for ResourceRecord {
    fn from(message: &MessageBuffer) ->  ResourceRecord {
        let mut resource_record = ResourceRecord::new();
        let mut current_index = 35;
        let compression_mask = 0b1100_0000;
        if message.buffer[current_index] == compression_mask {
            let offset_mask = 0b0011_1111_1111_1111;
            let mut offset: u16 = 0;
            offset += message.buffer[current_index] as u16;
            offset <<= 8;
            offset |= message.buffer[current_index + 1] as u16;
            offset &= offset_mask;
            let mut name = String::from("");
            let mut offset = offset as usize;
            println!("{}", offset);

            while message.buffer[offset] != 0 {
                let qname_count = message.buffer[offset];

                for byte in 1..=qname_count {
                    let index = offset + byte as usize;
                    let character = message.buffer[index] as char;
                    resource_record.name.push(character);
                }

                offset += qname_count as usize;
                offset += 1;

                if message.buffer[offset] != 0 {
                    resource_record.name.push('.');
                }
            }

        }

        current_index += 2;
        let mut type_value: u16 = 0;
        type_value += message.buffer[current_index] as u16;
        type_value <<= 8;
        current_index += 1;
        type_value |= message.buffer[current_index] as u16;

        match type_value {
            1 => resource_record.rtype = Type::A,
            2 => resource_record.rtype = Type::NS,
            3 => resource_record.rtype = Type::MD,
            4 => resource_record.rtype = Type::MF,
            5 => resource_record.rtype = Type::CNAME,
            16 => resource_record.rtype = Type::TXT,
            _ => resource_record.rtype = Type::NULL
        }

        current_index += 1;
        let mut class_value: u16 = 0;
        class_value += message.buffer[current_index] as u16;
        class_value <<= 8;
        current_index += 1;
        class_value |= message.buffer[current_index] as u16;

        match class_value {
            1 => resource_record.rclass = Class::IN,
            2 => resource_record.rclass = Class::CS,
            3 => resource_record.rclass = Class::CH,
            4 => resource_record.rclass = Class::HS,
            _ => resource_record.rclass = Class::IN
        }

        current_index += 1;

        resource_record.ttl |= message.buffer[current_index] as u32;
        resource_record.ttl <<= 24;
        current_index+=1;
        resource_record.ttl |= message.buffer[current_index] as u32;
        resource_record.ttl <<= 16;
        current_index+=1;
        resource_record.ttl |= message.buffer[current_index] as u32;
        resource_record.ttl <<= 8;
        current_index+=1;
        resource_record.ttl |= message.buffer[current_index] as u32;
        current_index+=1;

        resource_record.rdlength |= message.buffer[current_index] as u16;
        resource_record.rdlength <<= 8;
        current_index+=1;
        resource_record.rdlength |= message.buffer[current_index] as u16;
        current_index+=1;

        for _ in 0..resource_record.rdlength {
            let value = message.buffer[current_index];
            resource_record.rdata.push_str(value.to_string().as_str());
            resource_record.rdata.push('.');
            current_index += 1;
        }

        println!("{}", current_index);

        resource_record.rdata.pop();

        return resource_record;
    }
}
