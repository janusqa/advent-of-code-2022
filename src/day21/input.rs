use std::error::Error;
use std::fs;

pub fn get_input(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    return Ok(contents);
}
