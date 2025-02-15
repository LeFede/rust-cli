use crate::*;
use colored::*;
use std::{
    fs::{self},
    path::Path,
};

pub fn create_dir_all(path: &str) {
    let components_dir = Path::new(path);
    if !components_dir.exists() {
        if let Err(e) = fs::create_dir_all(components_dir) {
            log_err!("Error creating folder. {}", e);
        } else {
            log_ok!("Succesfully created folder \"{}\"", path);
        }
        return ();
    } else {
        log_info!("Folder \"{}\" already exists", path);
    }
}
