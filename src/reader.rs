use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut con = String::new();
    file.read_to_string(&mut con)?;
    Ok(con)
}
