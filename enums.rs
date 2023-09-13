#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Type {
    #[default]
    A,
    AAAA,
    NS,
    CNAME,
    NULL,
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 =>  Type::A,
            2 =>  Type::NS,
            5 =>  Type::CNAME,
            28 => Type::AAAA,
            _ =>  Type::NULL
        }
    }
}

impl From<Type> for [u8; 2] {
    fn from(value: Type) -> Self {
        match value {
            Type::A =>     [0b0000_0000, 0b0000_0001],
            Type::AAAA =>  [0b0000_0000, 0b0001_1100],
            Type::NS =>    [0b0000_0000, 0b0000_0010],
            Type::CNAME => [0b0000_0000, 0b0000_0101],
            Type::NULL =>  [0b0000_0000, 0b0000_1010],
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

