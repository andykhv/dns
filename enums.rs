#[derive(Debug, Default)]
pub enum Type {
    #[default]
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

impl From<Type> for [u8; 2] {
    fn from(value: Type) -> Self {
        match value {
            Type::A =>     [0b0000_0000, 0b0000_0001],
            Type::NS =>    [0b0000_0000, 0b0000_0010],
            Type::MD =>    [0b0000_0000, 0b0000_0011],
            Type::MF =>    [0b0000_0000, 0b0000_0100],
            Type::CNAME => [0b0000_0000, 0b0000_0101],
            Type::SOA =>   [0b0000_0000, 0b0000_0110],
            Type::MB =>    [0b0000_0000, 0b0000_0111],
            Type::MG =>    [0b0000_0000, 0b0000_1000],
            Type::MR =>    [0b0000_0000, 0b0000_1001],
            Type::NULL =>  [0b0000_0000, 0b0000_1010],
            Type::WKS =>   [0b0000_0000, 0b0000_1011],
            Type::PTR =>   [0b0000_0000, 0b0000_1100],
            Type::HINFO => [0b0000_0000, 0b0000_1101],
            Type::MINFO => [0b0000_0000, 0b0000_1110],
            Type::MX =>    [0b0000_0000, 0b0000_1111],
            Type::TXT =>   [0b0000_0000, 0b0001_0000],
        }
    }
}

#[derive(Debug, Default)]
pub enum Class {
    #[default]
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<Class> for [u8; 2] {
    fn from(value: Class) -> Self {
        match value {
            Class::IN => [0b0000_0000, 0b0000_0001],
            Class::CS => [0b0000_0000, 0b0000_0010],
            Class::CH => [0b0000_0000, 0b0000_0011],
            Class::HS => [0b0000_0000, 0b0000_0100],
        }
    }
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

