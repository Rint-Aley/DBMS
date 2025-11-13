pub mod structures;

use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::windows::fs::FileExt;
use std::path::Path;
use structures::*;
use tokio::net::tcp::ReuniteError;

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
    let free_cells = FreeSpace::new(0, 0, free_cells_available).unwrap();
    free_space.write_all(&free_cells.serialize()).unwrap();

    let pages = table_directory.join(PAGES_DIRECTORY_NAME);
    fs::create_dir(&pages).unwrap();
    let page = File::create(pages.join("0")).unwrap();
    page.set_len(PAGE_SIZE_BYTES).unwrap();
}

pub fn add_records(table_path: &str, records: &[&[Type]]) -> Result<(), &'static str> {
    if !is_table_exists(table_path) {
        return Err("Table is corrupted or does not exist.");
    }

    let table_path = Path::new(table_path);
    let pages_dir_path = table_path.join(PAGES_DIRECTORY_NAME);

    let raw_data = fs::read(table_path.join(DESCRIPTION_FILE_NAME)).unwrap();
    let table_metadata = TableMetadata::deserialize(&raw_data).unwrap();

    // validate 'values' vector
    if records
        .iter()
        .all(|record| record.len() as u16 == table_metadata.record_size())
    {}
    //values.iter().all(|record| record.iter().all(|value| value.type_id() == table_metadata.));

    let raw_data = fs::read(table_path.join(FREE_SPACE_FILE_NAME)).unwrap();
    let mut free_space_data = FreeSpace::deserialize_multiple(&raw_data).unwrap();

    let free_cells: u16 = free_space_data
        .iter()
        .map(|free_space| free_space.free_space())
        .sum();

    let required_cells = records.len() as u16;

    if free_cells < required_cells {
        let records_per_page = PAGE_SIZE_BYTES as u16 / table_metadata.record_size();
        let new_pages_required = required_cells.div_ceil(records_per_page);
        let current_number_of_pages: u16 = 0; // todo
        for i in current_number_of_pages..(current_number_of_pages + new_pages_required) {
            File::create(pages_dir_path.join(i.to_string())).unwrap();
            free_space_data.push(FreeSpace::new(i, 0, records_per_page)?);
        }
    }

    let mut free_cell_idx: usize = 0;
    let mut current_record: usize = 0;
    while current_record < records.len() {
        let records_to_add = std::cmp::min(
            (records.len() - current_record) as u16,
            free_space_data[free_cell_idx].free_space(),
        );

        let mut page =
            File::open(pages_dir_path.join(free_space_data[free_cell_idx].page.to_string()))
                .unwrap();
        write_records_in_row(
            &mut page,
            &records[current_record..(current_record + records_to_add as usize)],
            free_space_data[free_cell_idx].begin(),
            table_metadata.record_size(),
        )
        .unwrap();

        if let Ok(free_cell_empty) = free_space_data[free_cell_idx].move_begining(records_to_add)
            && free_cell_empty
        {
            free_cell_idx += 1;
        }
        current_record += records_to_add as usize;
    }

    fs::write(
        table_path.join(DESCRIPTION_FILE_NAME),
        FreeSpace::serialize_multiple(&free_space_data[free_cell_idx..]),
    )
    .unwrap();

    // todo: updating indexes (including pk)
    Ok(())
}

fn write_records_in_row(
    file: &mut File,
    values: &[&[Type]],
    position: u16,
    record_size: u16,
) -> Result<(), std::io::Error> {
    file.seek(SeekFrom::Start((position * record_size) as u64))?;
    for value in values {
        file.write_all(&structures::dbtype::serialize_values(value))
            .unwrap();
    }
    Ok(())
}

fn delete_records_by_position(
    table_path: &str,
    positions: &[DataPosition],
) -> Result<(), &'static str> {
    if positions.is_empty() {
        return Ok(());
    }

    let table_dir = Path::new(table_path);
    let free_space_path = table_dir.join(FREE_SPACE_FILE_NAME);
    let mut current_free_cells =
        FreeSpace::deserialize_multiple(&fs::read(&free_space_path).unwrap()).unwrap();
    let mut new_free_cells: Vec<FreeSpace> = Vec::new();
    current_free_cells.extend_from_slice(
        &positions
            .iter()
            .map(|position| {
                FreeSpace::new(position.page, position.cell, position.cell + 1).unwrap()
            })
            .collect::<Vec<FreeSpace>>(),
    );

    current_free_cells.sort_by_key(|cell| (cell.page, cell.begin()));
    let mut free_section = FreeSpace::new(
        current_free_cells[0].page,
        current_free_cells[0].begin(),
        current_free_cells[0].end(),
    )?;

    for value in current_free_cells.iter().skip(1) {
        if value.page != free_section.page || free_section.end() < value.begin() {
            new_free_cells.push(free_section.clone());
            free_section = value.clone();
            continue;
        } else if value.end() > free_section.end() {
            free_section.extend_end(value.end() - free_section.end());
        }
    }
    new_free_cells.push(free_section);

    fs::write(
        &free_space_path,
        FreeSpace::serialize_multiple(&new_free_cells),
    )
    .unwrap();
    Ok(())
}

fn get_records_by_position(
    table_path: &str,
    positions: &[DataPosition],
    record_structure: &[Type],
    record_size: u16,
) -> Result<Vec<Vec<Type>>, &'static str> {
    if positions.is_empty() {
        return Ok(Vec::new());
    }

    let page_dir = Path::new(table_path).join(PAGES_DIRECTORY_NAME);
    let mut positions = positions.to_vec();
    positions.sort_by_key(|position| (position.page, position.cell));

    let mut current_page_idx = positions[0].page;
    let page_path = page_dir.join(positions[0].page.to_string());
    let mut current_page = File::open(page_path).unwrap();
    let mut values = Vec::with_capacity(positions.len());
    for position in positions {
        if position.page != current_page_idx {
            let page_path = page_dir.join(position.page.to_string());
            current_page = File::open(page_path).unwrap();
            current_page_idx = position.page;
        }
        current_page
            .seek(SeekFrom::Start(position.cell as u64))
            .unwrap();
        let mut buffer = Vec::with_capacity(record_size as usize);
        current_page.read_exact(&mut buffer).unwrap();
        values.push(structures::dbtype::deserialize_value(&buffer, record_structure).unwrap());
    }

    Ok(values)
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
