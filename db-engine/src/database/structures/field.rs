use super::Type;
use byteorder::ReadBytesExt;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub type_: Type,
    pub nullable: bool,
}

impl Field {
    pub fn serialize(&self) -> Vec<u8> {
        // todo: either make name length unlimited or make a good error handling
        let length = self.name.len();
        if length > u8::MAX.into() {
            panic!("Name of '{0}' field takes more than 255 bytes", self.name);
        }
        let mut buffer: Vec<u8> = vec![0; length + 3];
        buffer[0] = length as u8;
        buffer[1..(length + 1)].copy_from_slice(self.name.as_bytes());
        buffer[length + 1] = self.type_.type_id();
        buffer[length + 2] = self.nullable.into();
        buffer
    }

    pub fn deserialize<R: Read>(rdr: &mut R) -> Result<Self, &'static str> {
        let name_len = rdr.read_u8().map_err(|_| "Missing name length")? as usize;

        let mut name_buf = vec![0u8; name_len];
        rdr.read_exact(&mut name_buf)
            .map_err(|_| "Incomplete field name")?;
        let name = String::from_utf8(name_buf).map_err(|_| "Invalid UTF-8 in field name")?;

        let type_id = rdr.read_u8().map_err(|_| "Missing type id")?;
        let nullable = rdr.read_u8().map_err(|_| "Missing nullable flag")? != 0;

        Ok(Self {
            name,
            type_: Type::from_type_id(type_id),
            nullable,
        })
    }
}
