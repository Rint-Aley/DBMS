use database::structures::*;
use std::error::Error;
use std::path::Path;
use std::{env, u16};

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    let database_directory = match env::var("DB_DIRECTORY") {
        Ok(path) => path,
        Err(_) => "database".to_string(),
    };

    println!("{database_directory}");
    Ok(())
}
