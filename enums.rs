#[derive(Debug)]
pub enum Type {
    A       = 1,
    NS      = 2,
    MD      = 3,
    MF      = 4,
    CNAME   = 5,
    SOA     = 6,
    MB      = 7,
    MG      = 8,
    MR      = 9,
    NULL    = 10,
    WKS     = 11,
    PTR     = 12,
    HINFO   = 13,
    MINFO   = 14,
    MX      = 15,
    TXT     = 16
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Type::A,
            2 => Type::NS,
            3 => Type::MD,
            4 => Type::MF,
            5 => Type::CNAME,
            6 => Type::SOA,
            7 => Type::MB,
            8 => Type::MG,
            9 => Type::MR,
            11 => Type::WKS,
            12 => Type::PTR,
            13 => Type::HINFO,
            14 => Type::MINFO,
            15 => Type::MX,
            16 => Type::TXT,
            _ => Type::NULL
        }
    }
}

#[derive(Debug)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Class::IN,
            2 => Class::CS,
            3 => Class::CH,
            4 => Class::HS,
            _ => Class::IN
        }
    }
}

