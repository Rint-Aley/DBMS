use super::Field;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableMetadata {
    fields: Vec<Field>,
    record_size: u16,
    primary_key_index: u16,
    indexes: Vec<u16>,
}

impl TableMetadata {
    pub fn new(
        fields: Vec<Field>,
        primary_key_index: u16,
        indexes: Vec<u16>,
    ) -> Result<Self, &'static str> {
        if primary_key_index as usize >= fields.len() {
            return Err("Wrong primary key location");
        } else if fields[primary_key_index as usize].nullable {
            return Err("Primary key cannot be nullable");
        }
        for &index in &indexes {
            if index as usize >= fields.len() {
                return Err("Wrong primary key location");
            }
        }
        let record_size = fields.iter().map(|f| f.type_.size() as u16).sum();
        Ok(TableMetadata {
            fields,
            record_size,
            primary_key_index,
            indexes,
        })
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        let mut rdr = Cursor::new(data);

        let field_count = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| "Corrupted data: missing field count")? as usize;

        let mut fields = Vec::with_capacity(field_count);

        for _ in 0..field_count {
            fields.push(Field::deserialize(&mut rdr)?);
        }

        let record_size = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| "Corrupted data: missing record_size")?;

        let primary_key_index = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| "Corrupted data: missing primary_key_index")?;

        let index_count = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| "Corrupted data: missing index count")? as usize;

        let mut indexes = Vec::with_capacity(index_count);
        for _ in 0..index_count {
            let idx = rdr
                .read_u16::<LittleEndian>()
                .map_err(|_| "Corrupted data: incomplete index list")?;
            indexes.push(idx);
        }

        Ok(TableMetadata {
            fields,
            record_size,
            primary_key_index,
            indexes,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ()> {
        let mut descriptor_content = Vec::<u8>::new();
        descriptor_content.extend_from_slice(&(self.fields.len() as u16).to_le_bytes());
        for field in &self.fields {
            descriptor_content.extend_from_slice(&field.serialize());
        }
        descriptor_content.extend_from_slice(&self.record_size.to_le_bytes());
        descriptor_content.extend_from_slice(&self.primary_key_index.to_le_bytes());
        descriptor_content.extend_from_slice(&(self.indexes.len() as u16).to_le_bytes());
        for index in &self.indexes {
            descriptor_content.extend_from_slice(&index.to_le_bytes());
        }
        Ok(descriptor_content)
    }

    pub fn record_size(&self) -> u16 {
        self.record_size
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn primary_key(&self) -> u16 {
        self.primary_key_index
    }

    pub fn indexes(&self) -> Vec<&Field> {
        self.indexes
            .iter()
            .map(|index| &self.fields[*index as usize])
            .collect()
    }

    pub fn indexes_idx(&self) -> &[u16] {
        &self.indexes
    }
}
