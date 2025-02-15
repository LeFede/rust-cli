use crate::*;
use colored::*;
use std::{
    fs::{self},
    path::Path,
};

pub fn create_file(path: &str, content: &str) {
    let main_file_path = Path::new(path);

    if main_file_path.exists() {
        file_already_exists!(path);
        return;
    }

    if let Err(e) = fs::write(main_file_path, content) {
        file_create_error!(path, e);
    } else {
        file_create_success!(path);
    }
}
