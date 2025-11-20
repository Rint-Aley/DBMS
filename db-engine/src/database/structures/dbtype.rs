use bincode::{Decode, Encode};
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Debug, Clone, Encode, Decode)]
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

    fn serialize(&self) -> Vec<u8> {
        macro_rules! num_to_bytes {
            ($var:expr) => {
                $var.to_le_bytes().to_vec()
            };
        }
        match self {
            Type::I8(var) => num_to_bytes!(var),
            Type::U8(var) => num_to_bytes!(var),
            Type::Boolean(var) => vec![*var as u8],
            Type::I16(var) => num_to_bytes!(var),
            Type::U16(var) => num_to_bytes!(var),
            Type::I32(var) => num_to_bytes!(var),
            Type::U32(var) => num_to_bytes!(var),
            Type::I64(var) => num_to_bytes!(var),
            Type::U64(var) => num_to_bytes!(var),
            Type::I128(var) => num_to_bytes!(var),
            Type::U128(var) => num_to_bytes!(var),
            Type::Varchar(_, var) => var.as_bytes().to_vec(),
        }
    }

    pub fn deserialize(data: &[u8], type_: &Self) -> Result<Self, &'static str> {
        if data.len() < type_.size() as usize {
            return Err("Length of the buffer is less than length of expected type.");
        }
        match type_ {
            Type::Boolean(_) => Ok(Type::Boolean(data[0] != 0)),
            Type::I8(_) => Ok(Type::I8(i8::from_le_bytes([data[0]]))),
            Type::U8(_) => Ok(Type::U8(u8::from_le_bytes([data[0]]))),
            Type::I16(_) => Ok(Type::I16(i16::from_le_bytes(data[..2].try_into().unwrap()))),
            Type::U16(_) => Ok(Type::U16(u16::from_le_bytes(data[..2].try_into().unwrap()))),
            Type::I32(_) => Ok(Type::I32(i32::from_le_bytes(data[..4].try_into().unwrap()))),
            Type::U32(_) => Ok(Type::U32(u32::from_le_bytes(data[..4].try_into().unwrap()))),
            Type::I64(_) => Ok(Type::I64(i64::from_le_bytes(data[..8].try_into().unwrap()))),
            Type::U64(_) => Ok(Type::U64(u64::from_le_bytes(data[..8].try_into().unwrap()))),
            Type::I128(_) => Ok(Type::I128(i128::from_le_bytes(
                data[..16].try_into().unwrap(),
            ))),
            Type::U128(_) => Ok(Type::U128(u128::from_le_bytes(
                data[..16].try_into().unwrap(),
            ))),
            Type::Varchar(len, _) => Ok(Type::Varchar(
                *len,
                std::str::from_utf8(&data[..(*len as usize)])
                    .unwrap()
                    .to_string(),
            )),
        }
    }

    fn data_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Type::I8(a), Type::I8(b)) => a.cmp(b),
            (Type::I16(a), Type::I16(b)) => a.cmp(b),
            (Type::I32(a), Type::I32(b)) => a.cmp(b),
            (Type::I64(a), Type::I64(b)) => a.cmp(b),
            (Type::I128(a), Type::I128(b)) => a.cmp(b),
            (Type::U8(a), Type::U8(b)) => a.cmp(b),
            (Type::U16(a), Type::U16(b)) => a.cmp(b),
            (Type::U32(a), Type::U32(b)) => a.cmp(b),
            (Type::U64(a), Type::U64(b)) => a.cmp(b),
            (Type::U128(a), Type::U128(b)) => a.cmp(b),
            (Type::Varchar(len_a, s_a), Type::Varchar(len_b, s_b)) => {
                // Compare length first, then string
                match len_a.cmp(len_b) {
                    Ordering::Equal => s_a.cmp(s_b),
                    ordering => ordering,
                }
            }
            (Type::Boolean(a), Type::Boolean(b)) => a.cmp(b),
            _ => Ordering::Equal, // Should never happen since we check discriminant first
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id() && self.data_cmp(other) == Ordering::Equal
    }
}

impl Eq for Type {}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_id().cmp(&other.type_id()) {
            Ordering::Equal => self.data_cmp(other),
            ordering => ordering,
        }
    }
}

pub fn serialize_values(data: &[Type]) -> Vec<u8> {
    let size = data.iter().map(|t| t.size() as usize).sum();
    let mut result = Vec::with_capacity(size);
    data.iter()
        .for_each(|data| result.extend_from_slice(&data.serialize()));
    result
}

pub fn deserialize_value(
    data: &[u8],
    record_structure: &[Type],
) -> Result<Vec<Type>, &'static str> {
    let record_len = record_structure.iter().map(|t| t.size() as usize).sum();
    if data.len() < record_len {
        return Err("Buffer is too short.");
    }
    let mut result = Vec::with_capacity(record_structure.len());
    let mut cursor = 0;
    for t in record_structure {
        result.push(Type::deserialize(
            &data[cursor..(cursor + t.size() as usize)],
            t,
        )?);
        cursor += t.size() as usize;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_deserialization() {
        let test_cases = vec![
            Type::I8(42),
            Type::U8(100),
            Type::Boolean(true),
            Type::Boolean(false),
            Type::I16(1000),
            Type::U16(2000),
            Type::I32(100_000),
            Type::U32(200_000),
            Type::I64(1_000_000),
            Type::U64(2_000_000),
            Type::I128(10_000_000),
            Type::U128(20_000_000),
            Type::Varchar(5, String::from("Hello")),
            Type::Varchar(0, String::new()),
        ];

        for value in test_cases {
            let serialized = value.serialize();
            let deserialized = Type::deserialize(&serialized, &value).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    fn deserialize_errors() {
        let value = Type::I32(42);
        let serialized = value.serialize();
        assert!(Type::deserialize(&serialized[..2], &value).is_err());

        // Varchar with wrong length
        let varchar = Type::Varchar(5, String::from("Hello"));
        let short_data = b"Hi";
        assert!(Type::deserialize(short_data, &varchar).is_err());
    }
}
