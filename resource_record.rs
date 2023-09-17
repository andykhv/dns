use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use crate::message_buffer::MessageBuffer;
use crate::enums::{Type, Class};

#[derive(Debug, Default, Clone)]
pub struct ResourceRecord {
    pub name: String,   //domain name
    pub rtype: Type,    //type code of rdata
    pub rclass: Class,  //class of rdata
    pub ttl: u32,       //time interval (seconds) until cache -> trash
    pub rdlength: u16,  //length of rdata
    pub rdata: String   //describes the resource
}

impl From<&mut MessageBuffer> for ResourceRecord {
    fn from(message: &mut MessageBuffer) ->  ResourceRecord {
        let mut resource_record = ResourceRecord::default();

        resource_record.name = ResourceRecord::read_domain_name(message);
        resource_record.rtype = ResourceRecord::read_type(message);
        resource_record.rclass = ResourceRecord::read_class(message);
        resource_record.ttl = ResourceRecord::read_ttl(message);
        resource_record.rdlength = ResourceRecord::read_length(message);
        resource_record.rdata = ResourceRecord::read_rdata(&resource_record.rtype, message);

        return resource_record;
    }
}

impl ResourceRecord {
    fn read_domain_name(message: &mut MessageBuffer) -> String {
        let mut name = String::new();
        let mut byte = message.next().unwrap_or_default();

        while byte != 0 {
            if ResourceRecord::is_pointer(byte) {
                let pointer = ResourceRecord::read_pointer((byte, message.next().unwrap_or_default()));
                let previous_pointer = message.get_position();
                let result = message.seek(pointer as usize); //this can panic since pointer is u16

                if result.is_err() {
                    println!("{}", result.unwrap_err());
                    return name;
                }

                name.push_str(&ResourceRecord::read_domain_name(message).as_str());

                let _ = message.seek(previous_pointer); //ignore error here since previous_pointer should be valid
                byte = 0; //end loop
            } else {
                name.push_str(ResourceRecord::read_label(byte, message).as_str());
                byte = message.next().unwrap_or_default();

                if byte == 0 {
                    name.pop();
                }
            }
        }

        return name;
    }

    fn is_pointer(byte: u8) -> bool {
        let compression_mask = 0b1100_0000;
        return (byte & compression_mask) == compression_mask;
    }

    //assume bytes is big endian
    fn read_pointer(bytes: (u8, u8)) -> u16 {
        let pointer_mask = 0b0011_1111_1111_1111;
        let mut pointer: u16 = 0;
        pointer |= bytes.0 as u16;
        pointer <<= 8;
        pointer |= bytes.1 as u16;
        pointer &= pointer_mask;

        return pointer;
    }

    fn read_label(label_count: u8, message: &mut MessageBuffer) -> String {
        let mut label = String::new();

        for _ in 0..label_count {
            let character = message.next().unwrap_or_default() as char;
            label.push(character);
        }

        label.push('.');

        return label;
    }

    fn read_type(message: &mut MessageBuffer) -> Type {
        let type_value = message.next_u16().unwrap_or_default();
        return Type::from(type_value);
    }

    fn read_class(message: &mut MessageBuffer) -> Class {
        let class_value = message.next_u16().unwrap_or_default();
        return Class::from(class_value);
    }

    fn read_ttl(message: &mut MessageBuffer) -> u32 {
        return message.next_u32().unwrap_or_default();
    }

    fn read_length(message: &mut MessageBuffer) -> u16 {
        return message.next_u16().unwrap_or_default();
    }

    fn read_rdata(rtype: &Type, message: &mut MessageBuffer) -> String {
        match rtype {
            Type::A     => ResourceRecord::read_ipv4_address(message),
            Type::AAAA  => ResourceRecord::read_ipv6_address(message),
            Type::CNAME => ResourceRecord::read_domain_name(message),
            Type::NS    => ResourceRecord::read_domain_name(message),
            _ => String::from("UNKNOWN TYPE")
        }
    }

    fn read_ipv4_address(message: &mut MessageBuffer) -> String {
        let ipv4 = Ipv4Addr::new(
            message.next().unwrap_or_default(),
            message.next().unwrap_or_default(),
            message.next().unwrap_or_default(),
            message.next().unwrap_or_default()
        );

        return ipv4.to_string();
    }

    fn read_ipv6_address(message: &mut MessageBuffer) -> String {
        let ipv6 = Ipv6Addr::new(
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
            message.next_u16().unwrap_or_default(),
        );

        return ipv6.to_string();
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for word in self.name.split('.') {
            bytes.push(word.len() as u8);
            bytes.append(&mut word.as_bytes().to_vec());
        }
        bytes.push(0);

        bytes.append(&mut <[u8; 2]>::from(self.rtype).to_vec());
        bytes.append(&mut <[u8; 2]>::from(self.rclass).to_vec());
        bytes.append(&mut self.ttl.to_be_bytes().to_vec());
        bytes.append(&mut self.rdlength.to_be_bytes().to_vec());

        if self.rtype == Type::A {
            let ipv4 = Ipv4Addr::from_str(&self.rdata).unwrap();
            bytes.append(&mut ipv4.octets().to_vec());
        } else if self.rtype == Type::AAAA {
            let ipv6 = Ipv6Addr::from_str(&self.rdata).unwrap();
            bytes.append(&mut ipv6.octets().to_vec());
        } else {
            for word in self.rdata.split('.') {
                bytes.push(word.len() as u8);
                bytes.append(&mut word.as_bytes().to_vec());
            }
        }

        return bytes;
    }
}
