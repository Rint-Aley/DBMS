#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    //String(String),
    I8(i8) = 1,
    I16(i16) = 2,
    I32(i32) = 3,
    I64(i64) = 4,
    I128(i128) = 5,
    U8(u8) = 6,
    U16(u16) = 7,
    U32(u32) = 8,
    U64(u64) = 9,
    U128(u128) = 10,
    Varchar(u8, String) = 11,
    Boolean(bool) = 12,
}

impl Type {
    pub fn type_id(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    pub fn size(&self) -> u8 {
        match self {
            Type::I8(_) | Type::U8(_) | Type::Boolean(_) => 1,
            Type::I16(_) | Type::U16(_) => 2,
            Type::I32(_) | Type::U32(_) => 3,
            Type::I64(_) | Type::U64(_) => 4,
            Type::I128(_) | Type::U128(_) => 5,
            Type::Varchar(len, _) => *len,
        }
    }

    pub fn from_type_id(id: u8) -> Self {
        match id {
            1 => Type::I8(0),
            2 => Type::I16(0),
            3 => Type::I32(0),
            4 => Type::I64(0),
            5 => Type::I128(0),
            6 => Type::U8(0),
            7 => Type::U16(0),
            8 => Type::U32(0),
            9 => Type::U64(0),
            10 => Type::U128(0),
            11 => Type::Varchar(255, String::new()), // default varchar length
            12 => Type::Boolean(false),
            _ => panic!("Unknown type id: {}", id),
        }
    }
}
