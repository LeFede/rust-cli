use std::fs::File;
use std::io::{self, Read};

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
