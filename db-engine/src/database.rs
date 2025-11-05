pub mod structures;

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use structures::*;

const DESCRIPTION_FILE_NAME: &str = "descriptor";
const FREE_SPACE_FILE_NAME: &str = "free_space";
const PAGES_DIRECTORY_NAME: &str = "pages";
const PAGE_SIZE_BYTES: u64 = 4096;

pub fn create_table(database_path: &str, name: &str, metadata: TableMetadata) {
    let table_directory = Path::new(database_path).join(name);
    if let Err(e) = fs::create_dir_all(&table_directory) {
        dbg!(e);
    };

    let descriptor_path = table_directory.join(DESCRIPTION_FILE_NAME);
    let mut descriptor = File::create(descriptor_path).unwrap();
    descriptor
        .write_all(&metadata.serialize().unwrap())
        .unwrap();

    let free_space_path = table_directory.join(FREE_SPACE_FILE_NAME);
    let mut free_space = File::create(free_space_path).unwrap();
    let free_cells_available = (PAGE_SIZE_BYTES as u16) / metadata.record_size();
    let free_cells = FreeSpace::new(1, 1, free_cells_available).unwrap();
    free_space.write_all(&free_cells.serialize()).unwrap();

    let pages = table_directory.join(PAGES_DIRECTORY_NAME);
    fs::create_dir(&pages).unwrap();
    let page = File::create(pages.join("1")).unwrap();
    page.set_len(PAGE_SIZE_BYTES).unwrap();
}

pub fn add_field(table_path: &str, new_field: Field) -> Result<(), Box<dyn Error>> {
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

fn add_record(table_path: &str, values: Vec<Type>) -> Result<(), &'static str> {
    let table_path = Path::new(table_path);

    unimplemented!();
    // updating indexes (including pk)
}

fn get_records(
    table_path: &str,
    filters: Vec<FilterOption>,
) -> Result<Vec<Vec<Type>>, Box<dyn Error>> {
    unimplemented!();
}

// Checks that overall table structure is valid
fn is_table_exists(table_path: &str) -> bool {
    let table_dir = Path::new(table_path);
    let descriptor = table_dir.join(DESCRIPTION_FILE_NAME);
    let free_space = table_dir.join(FREE_SPACE_FILE_NAME);
    let pages = table_dir.join(PAGES_DIRECTORY_NAME);
    table_dir.exists() && descriptor.exists() && free_space.exists()
}
