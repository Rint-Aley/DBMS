use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use prost::bytes::BufMut;

const DESCRIPTION_FILE_NAME: &str = "descriptor";
const FREE_SPACE_FILE_NAME: &str = "free_space";
const PAGE_SIZE_BYTES: u64 = 4096;

struct TableMetadata {
    fields: Vec<Field>,
    record_size: u16,
    primary_key_index: u16,
    indexes_index: Vec<u16>,
}

#[repr(u8)]
enum Type {
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
    fn type_id(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn size(&self) -> u8 {
        match self {
            Type::I8(_) | Type::U8(_) | Type::Boolean(_) => 1,
            Type::I16(_) | Type::U16(_) => 2,
            Type::I32(_) | Type::U32(_) => 3,
            Type::I64(_) | Type::U64(_) => 4,
            Type::I128(_) | Type::U128(_) => 5,
            Type::Varchar(len, _) => *len,
        }
    }
}

struct Field {
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
}

enum Filter {
    Equal,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Contains,
    StratsWith,
    EndsWith,
}

struct FilterOption {
    filed: Field,
    filter: Filter,
}

fn validate_table_info(
    fields: &[Field],
    primary_key_index: u16,
    indexes: &[u16],
) -> Result<(), &'static str> {
    if primary_key_index as usize >= fields.len() {
        return Err("Wrong primary key location");
    } else if fields[primary_key_index as usize].nullable {
        return Err("Primary key cannot be nullable");
    }
    for &index in indexes {
        if index as usize >= fields.len() {
            return Err("Wrong primary key location");
        }
    }
    Ok(())
}

fn create_table(
    database_path: &str,
    name: &str,
    fields: &[Field],
    primary_key_index: u16,
    indexes: &[u16],
) {
    if let Err(e) = validate_table_info(fields, primary_key_index, indexes) {
        panic!("{e}");
    }

    let table_directory = Path::new(database_path).join(name);
    if let Err(e) = fs::create_dir_all(&table_directory) {
        dbg!(e);
    };

    let descriptor_path = table_directory.join(DESCRIPTION_FILE_NAME);
    let mut descriptor = File::create(descriptor_path).unwrap();
    let mut descriptor_content = Vec::<u8>::new();
    let mut record_size: u16 = 0;
    descriptor_content.extend_from_slice(&(fields.len() as u16).to_le_bytes());
    for field in fields {
        descriptor_content.extend_from_slice(&field.serialize());
        record_size += field.type_.size() as u16;
    }
    descriptor_content.extend_from_slice(&record_size.to_le_bytes());
    descriptor_content.extend_from_slice(&primary_key_index.to_le_bytes());
    descriptor_content.extend_from_slice(&(indexes.len() as u16).to_le_bytes());
    for index in indexes {
        descriptor_content.extend_from_slice(&index.to_le_bytes());
    }
    descriptor.write_all(&descriptor_content).unwrap();

    let free_space_path = table_directory.join(FREE_SPACE_FILE_NAME);
    let mut free_space = File::create(free_space_path).unwrap();
    let free_cells_available = PAGE_SIZE_BYTES / (record_size as u64);
    let free_space_info: [u64; 3] = [1, 1, free_cells_available];
    for &info in &free_space_info {
        free_space.write_all(&info.to_le_bytes()).unwrap();
    }

    let pages = table_directory.join("pages");
    fs::create_dir(&pages).unwrap();
    let page = File::create(pages.join("1")).unwrap();
    page.set_len(PAGE_SIZE_BYTES).unwrap();
}

fn is_table_exists(table_path: &str) -> bool {
    let table_directory = Path::new(table_path);
    table_directory.exists() && table_directory.join(DESCRIPTION_FILE_NAME).exists()
}

fn add_field(table_path: &str, new_field: Field) -> Result<(), Box<dyn Error>> {
    if !is_table_exists(table_path) {};
    if get_fields(table_path)
        .unwrap()
        .iter()
        .any(|field| field.name == new_field.name)
    {};
    // serialize new field
    unimplemented!();
}

fn get_fields(table_path: &str) -> Result<Vec<Field>, Box<dyn Error>> {
    unimplemented!();
}

fn add_record(table_path: &str, values: Vec<Type>) -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

fn get_records(
    table_path: &str,
    filters: Vec<FilterOption>,
) -> Result<Vec<Vec<Type>>, Box<dyn Error>> {
    unimplemented!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let database_directory = match env::var("DB_DIRECTORY") {
        Ok(path) => path,
        Err(_) => "database".to_string(),
    };

    println!("{database_directory}");
    Ok(())
}
