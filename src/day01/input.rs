use std::fs::File;
use std::io::{self, Read};

pub fn get_input(file_path: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    File::open(file_path)?.read_to_string(&mut input)?;

    return Ok(input);
}
