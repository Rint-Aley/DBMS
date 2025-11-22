pub mod structures;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::format;
use std::fs::OpenOptions;
use std::fs::{self, File, metadata};
use std::hash::Hash;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use structures::*;

use crate::database;

const DESCRIPTION_FILE_NAME: &str = "descriptor";
const FREE_SPACE_FILE_NAME: &str = "free_space";
const PAGES_DIRECTORY_NAME: &str = "pages";
const NUMBER_OF_PAGES_FILE_NAME: &str = "number";
const INDEXES_DIRECTORY_NAME: &str = "indexes";
const PAGE_SIZE_BYTES: u16 = 4096;

pub fn create_table(
    database_path: &str,
    name: &str,
    metadata: TableMetadata,
) -> Result<(), String> {
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
    page.set_len(PAGE_SIZE_BYTES as u64).unwrap();
    write_number_of_pages(&table_directory, 1).unwrap();

    let indexes_dir = table_directory.join(INDEXES_DIRECTORY_NAME);
    fs::create_dir(&indexes_dir).unwrap();
    for field in metadata.fields() {
        let map: BTreeMap<Type, Vec<DataPosition>> = BTreeMap::new();
        write_index(&indexes_dir.join(&field.name), &map)?;
        // if write_index will not create a new file then uncomment following code
        //let mut index_file = File::create(indexes_dir.join(&field.name)).unwrap();
        //let config = bincode::config::standard().with_little_endian();
        //bincode::encode_into_std_write(map, &mut index_file, config).unwrap();
    }
    Ok(())
}

pub fn clear_table(table_path: &Path) -> Result<(), String> {
    let free_space_path = table_path.join(FREE_SPACE_FILE_NAME);
    let metadata_path = table_path.join(DESCRIPTION_FILE_NAME);
    let pages_dir = table_path.join(PAGES_DIRECTORY_NAME);

    let raw_data = match fs::read(metadata_path) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Error while reading table metadata: {}", e));
        }
    };
    let metadata = TableMetadata::deserialize(&raw_data)?;

    let number_of_pages: u64 = read_number_of_pages(table_path)?;
    for i in 1..number_of_pages {
        let file_path = pages_dir.join(i.to_string());
        if let Err(e) = fs::remove_file(&file_path)
            && e.kind() == io::ErrorKind::PermissionDenied
        {
            return Err(format!(
                "Error while deleting page {}: permission denied",
                file_path.display()
            ));
        }
    }
    write_number_of_pages(table_path, 1)?;
    let number_of_cells_per_page = PAGE_SIZE_BYTES / metadata.record_size();
    if let Err(e) = fs::write(
        free_space_path,
        FreeSpace::new(0, 0, number_of_cells_per_page)
            .unwrap()
            .serialize(),
    ) {
        return Err(format!("Error while writing to free space file: {}", e));
    };
    Ok(())
}

pub fn delete_table(table_path: &Path) -> Result<(), String> {
    match fs::remove_dir_all(table_path) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// Not the primary concern for now
pub fn add_index() {
    unimplemented!()
}

// Not the primary concern for now
pub fn delete_index() {
    unimplemented!()
}

pub fn add_records(table_path: &str, records: &[&[Type]]) -> Result<(), String> {
    if !is_table_exists(table_path) {
        return Err(String::from("Table is corrupted or does not exist."));
    }

    let table_path = Path::new(table_path);
    let pages_dir = table_path.join(PAGES_DIRECTORY_NAME);
    let indexes_dir = table_path.join(INDEXES_DIRECTORY_NAME);
    let free_space_path = table_path.join(FREE_SPACE_FILE_NAME);

    let raw_data = fs::read(table_path.join(DESCRIPTION_FILE_NAME)).unwrap();
    let table_metadata = TableMetadata::deserialize(&raw_data).unwrap();

    // TODO: validate 'values' vector
    // if records
    //     .iter()
    //     .all(|record| record.len() as u16 == table_metadata.record_size())
    // {}
    //values.iter().all(|record| record.iter().all(|value| value.type_id() == table_metadata.));

    // Check new records for primary key uniqueness
    // TODO: check for pk duplicates in the new ones
    let pk_idx = table_metadata.primary_key() as usize;
    let pk_field = &table_metadata.fields()[pk_idx];
    let pk_index_path = indexes_dir.join(&pk_field.name);
    let pk_index = read_index(&pk_index_path)?;
    let all_records_unique = records
        .iter()
        .all(|&record| match pk_index.get(&record[pk_idx]) {
            Some(data) => data.is_empty(),
            None => true,
        });
    if !all_records_unique {
        return Err(String::from("Error: new records break the pk uniqueness."));
    }

    let raw_data = fs::read(&free_space_path).unwrap();
    let mut free_space_data = FreeSpace::deserialize_multiple(&raw_data).unwrap();

    let num_of_free_cells: u64 = free_space_data
        .iter()
        .map(|free_space| free_space.free_space() as u64)
        .sum();

    let required_num_of_free_cells = records.len() as u64;

    // If table lack of free space to store data it creates new pages
    if num_of_free_cells < required_num_of_free_cells {
        let records_per_page = PAGE_SIZE_BYTES / table_metadata.record_size();
        let new_pages_required = required_num_of_free_cells.div_ceil(records_per_page as u64);
        let current_number_of_pages = read_number_of_pages(table_path).unwrap();
        for i in current_number_of_pages..(current_number_of_pages + new_pages_required) {
            File::create(pages_dir.join(i.to_string())).unwrap();
            free_space_data.push(FreeSpace::new(i, 0, records_per_page)?);
        }
        write_number_of_pages(table_path, current_number_of_pages + new_pages_required).unwrap();
    }

    // Writing records
    let mut records_position: HashMap<usize, DataPosition> = HashMap::new();
    let mut free_cell_idx: usize = 0;
    let mut current_record: usize = 0;
    while current_record < records.len() {
        let records_to_add = std::cmp::min(
            (records.len() - current_record) as u16,
            free_space_data[free_cell_idx].free_space(),
        );
        let free_position_begining =
            free_space_data[free_cell_idx].begin() * table_metadata.record_size();

        let mut page =
            File::open(pages_dir.join(free_space_data[free_cell_idx].page.to_string())).unwrap();

        page.seek(SeekFrom::Start(free_position_begining as u64))
            .unwrap();
        let mut record_position = free_position_begining;
        for i in 0..records_to_add as usize {
            page.write_all(&structures::dbtype::serialize_values(
                records[current_record + i],
            ))
            .unwrap();
            records_position.insert(
                current_record + i,
                DataPosition {
                    page: free_space_data[free_cell_idx].page,
                    cell: record_position,
                },
            );
            record_position += table_metadata.record_size();
        }

        if let Ok(free_cell_empty) = free_space_data[free_cell_idx].move_begining(records_to_add)
            && free_cell_empty
        {
            free_cell_idx += 1;
        }
        current_record += records_to_add as usize;
    }

    fs::write(
        free_space_path,
        FreeSpace::serialize_multiple(&free_space_data[free_cell_idx..]),
    )
    .unwrap();

    // Updating indexes
    let fields = table_metadata.fields();
    //let config = bincode::config::standard().with_little_endian();
    for &index_idx in table_metadata.indexes_idx() {
        let index_name = &fields[index_idx as usize].name;
        // let mut index_file = File::open(indexes_dir.join(index_name)).unwrap();
        // let mut index_map: BTreeMap<Type, Vec<DataPosition>> =
        //     bincode::decode_from_std_read(&mut index_file, config).unwrap();
        let index_path = indexes_dir.join(index_name);
        let mut index_map = read_index(&index_path)?;
        for (i, &record) in records.iter().enumerate() {
            index_map
                .entry(record[index_idx as usize].clone())
                .or_insert_with(Vec::new)
                .push(records_position.get(&i).unwrap().clone());
        }
        // bincode::encode_into_std_write(index_map, &mut index_file, config).unwrap();
        write_index(&index_path, &index_map).unwrap();
    }
    Ok(())
}

pub fn get_records(
    table_path: &Path,
    filters: &[FilterOption],
) -> Result<Vec<Vec<Type>>, &'static str> {
    let descriptor = Path::new(table_path).join(DESCRIPTION_FILE_NAME);
    let metadata: TableMetadata =
        TableMetadata::deserialize(&fs::read(descriptor).unwrap()).unwrap();
    let positions = get_positions(table_path, filters).unwrap();
    let db_structure: Vec<Type> = metadata
        .fields()
        .iter()
        .map(|field| field.type_.clone())
        .collect();
    get_records_by_position(
        table_path,
        &positions,
        &db_structure,
        metadata.record_size(),
    )
}

pub fn delete_records(table_path: &Path, filters: &[FilterOption]) -> Result<(), String> {
    let pages_dir = table_path.join(PAGES_DIRECTORY_NAME);
    let metadata_path = table_path.join(DESCRIPTION_FILE_NAME);

    let raw_metadata_content = fs::read(metadata_path).unwrap();
    let metadata = TableMetadata::deserialize(&raw_metadata_content)?;

    let record_positions = get_positions(&table_path, filters)?;
    let record_positions_set: HashSet<DataPosition> =
        record_positions.clone().into_iter().collect();
    if record_positions.is_empty() {
        return Ok(());
    }

    // Delete indexes
    let fields = metadata.fields();
    let records_structure: Vec<Type> = fields.iter().map(|filed| filed.type_.clone()).collect();
    let record_size = records_structure
        .iter()
        .map(|field| field.size() as u16)
        .sum();
    let records = get_records_by_position(
        table_path,
        &record_positions,
        &records_structure,
        record_size,
    )
    .unwrap();

    for &index_idx in metadata.indexes_idx() {
        let index_idx = index_idx as usize;
        let values_range: BTreeSet<&Type> =
            records.iter().map(|record| &record[index_idx]).collect();
        let index_path = pages_dir.join(&fields[index_idx].name);
        let mut index = read_index(&index_path)?;
        for value in values_range {
            if let Some(positions) = index.get_mut(value) {
                positions.retain(|position| !record_positions_set.contains(position));
            }
        }
        write_index(&index_path, &index)?;
    }
    delete_records_by_position(table_path, &record_positions)
}

pub fn change_records() {
    // find records according to a filter
    // changed values
    // check for uniquenes in pk is changed
    unimplemented!()
}

fn get_positions(table_path: &Path, filters: &[FilterOption]) -> Result<Vec<DataPosition>, String> {
    let indexes_dir = table_path.join(INDEXES_DIRECTORY_NAME);
    let descriptor = table_path.join(DESCRIPTION_FILE_NAME);
    let metadata = TableMetadata::deserialize(&fs::read(descriptor).unwrap()).unwrap();

    if filters.is_empty() {
        return get_all_positions(table_path, &metadata);
    }

    // TODO: check if filter option is valid (fileds corresponds to the exicting ones)

    // if there are indexed filters check all of them first (if not than get all records)
    // for every other field do a linear search

    let indexed_fields: HashSet<String> = metadata
        .indexes()
        .iter()
        .map(|&field| field.name.clone())
        .collect();
    let mut indexed_filters = Vec::new();
    let mut unindexed_filters = Vec::new();
    for filter in filters {
        if indexed_fields.contains(&filter.field().name) {
            indexed_filters.push(filter);
        } else {
            unindexed_filters.push(filter);
        }
    }

    let mut positions: HashSet<DataPosition>;
    if indexed_filters.is_empty() {
        positions = get_all_positions(table_path, &metadata)
            .unwrap()
            .into_iter()
            .collect();
    } else {
        // TODO: Skip one iteration in for loop and move it here
        positions = get_all_positions(table_path, &metadata)
            .unwrap()
            .into_iter()
            .collect();
    }
    for filter in indexed_filters {
        let index_path = indexes_dir.join(&filter.field().name);
        let index = read_index(&index_path)?;
        match filter.filter() {
            Filter::Equal => {
                match index.get(&filter.field().type_) {
                    Some(new_positions) => {
                        if new_positions.is_empty() {
                            return Ok(Vec::new());
                        }
                        positions = positions
                            .intersection(&HashSet::from_iter(new_positions.clone().into_iter()))
                            .cloned()
                            .collect();
                    }
                    None => return Ok(Vec::new()),
                };
            }
            _ => unimplemented!(),
        }
    }
    for filter in unindexed_filters {
        linear_search(&mut positions, table_path, filter, &metadata).unwrap();
    }

    Ok(positions.into_iter().collect())
}

fn get_records_by_position(
    table_path: &Path,
    positions: &[DataPosition],
    record_structure: &[Type],
    record_size: u16,
) -> Result<Vec<Vec<Type>>, &'static str> {
    if positions.is_empty() {
        return Ok(Vec::new());
    }

    let page_dir = table_path.join(PAGES_DIRECTORY_NAME);
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

fn linear_search(
    data: &mut HashSet<DataPosition>,
    table_path: &Path,
    filter: &FilterOption,
    metadata: &TableMetadata,
) -> Result<(), &'static str> {
    let pages_dir = table_path.join(PAGES_DIRECTORY_NAME);
    let mut data_vec: Vec<DataPosition> = data.clone().into_iter().collect();
    data_vec.sort_by_key(|position| position.page);

    let record_structure: Vec<Type> = metadata
        .fields()
        .iter()
        .map(|field| field.type_.clone())
        .collect();
    let record_size: u16 = record_structure.iter().map(|t| t.size() as u16).sum();
    let mut number_of_filtered_field = 0;
    for (i, field) in metadata.fields().iter().enumerate() {
        if field.name == filter.field().name {
            number_of_filtered_field = i;
        }
    }

    let mut current_page_num = data_vec[0].page;
    let page_path = pages_dir.join(current_page_num.to_string());
    let mut page_content: Vec<u8> = fs::read(page_path).unwrap();

    for position in data_vec {
        if current_page_num != position.page {
            current_page_num = position.page;
            let page_path = pages_dir.join(current_page_num.to_string());
            page_content = fs::read(page_path).unwrap();
        }
        let record = dbtype::deserialize_value(
            &page_content[(position.cell as usize)..(position.cell + record_size) as usize],
            &record_structure,
        )
        .unwrap();
        if record[number_of_filtered_field] != filter.field().type_ {
            data.remove(&position);
        }
    }
    Ok(())
}

fn get_all_positions(
    table_path: &Path,
    metadata: &TableMetadata,
) -> Result<Vec<DataPosition>, String> {
    let free_space_path = table_path.join(FREE_SPACE_FILE_NAME);
    let num_of_pages = read_number_of_pages(table_path).unwrap();
    let record_size = metadata.record_size();
    let records_per_page = PAGE_SIZE_BYTES / record_size;

    let mut free_spaces =
        FreeSpace::deserialize_multiple(&fs::read(free_space_path).unwrap()).unwrap();
    free_spaces.sort_by_key(|free_space| (free_space.page, free_space.begin()));

    let mut free_space_idx = 0;
    let mut begining;
    let mut positions = Vec::new();
    let generate_positions = |page_num: u64, begin_position: u16, end_position: u16| {
        (begin_position..end_position)
            .map(|position| DataPosition {
                page: page_num,
                cell: position * record_size,
            })
            .collect()
    };
    // work with corner values
    for page_num in 0..num_of_pages {
        begining = 0;
        while free_spaces[free_space_idx].page == page_num {
            positions.append(&mut generate_positions(
                page_num,
                begining,
                free_spaces[free_space_idx].begin(),
            ));
            begining = free_spaces[free_space_idx].end();
            free_space_idx += 1;
        }
        if begining < records_per_page {
            positions.append(&mut generate_positions(
                page_num,
                begining,
                records_per_page,
            ));
        }
    }
    Ok(positions)
}

fn delete_records_by_position(table_path: &Path, positions: &[DataPosition]) -> Result<(), String> {
    // TODO: deleting page when it depletes (rename last page and update number of pages)
    if positions.is_empty() {
        return Ok(());
    }

    let free_space_path = table_path.join(FREE_SPACE_FILE_NAME);
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

fn read_number_of_pages(database_path: &Path) -> Result<u64, &'static str> {
    let number_of_pages_path = database_path.join(NUMBER_OF_PAGES_FILE_NAME);
    let buffer: [u8; 8] = fs::read(number_of_pages_path).unwrap().try_into().unwrap();
    Ok(u64::from_le_bytes(buffer))
}

fn write_number_of_pages(database_path: &Path, value: u64) -> Result<(), &'static str> {
    let number_of_pages_path = database_path.join(NUMBER_OF_PAGES_FILE_NAME);
    fs::write(number_of_pages_path, value.to_le_bytes()).unwrap();
    Ok(())
}

fn write_index(
    index_path: &Path,
    index_data: &BTreeMap<Type, Vec<DataPosition>>,
) -> Result<(), String> {
    let config = bincode::config::standard().with_little_endian();
    let open_options = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(index_path);
    let mut index_file = match open_options {
        Ok(file) => file,
        Err(_) => {
            return Err(format!(
                "Index file at {} is not accessable.",
                index_path.display()
            ));
        }
    };
    match bincode::encode_into_std_write(index_data, &mut index_file, config) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error while encoding index: {}", e)),
    }
}

fn read_index(index_path: &Path) -> Result<BTreeMap<Type, Vec<DataPosition>>, String> {
    let config: bincode::config::Configuration = bincode::config::standard().with_little_endian();
    let mut index_file = match File::open(index_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!(
                "Index file at {} is not accessable.",
                index_path.display()
            ));
        }
    };
    match bincode::decode_from_std_read(&mut index_file, config) {
        Ok(map) => Ok(map),
        Err(e) => Err(format!("Error while decoding index: {}", e)),
    }
}
// Checks that overall table structure is valid
fn is_table_exists(table_path: &str) -> bool {
    let table_dir = Path::new(table_path);
    let descriptor = table_dir.join(DESCRIPTION_FILE_NAME);
    let free_space = table_dir.join(FREE_SPACE_FILE_NAME);
    let pages = table_dir.join(PAGES_DIRECTORY_NAME);
    table_dir.exists() && descriptor.exists() && free_space.exists()
}
