pub mod log_err;
pub mod log_info;
pub mod log_ok;

#[macro_export]
macro_rules! file_already_exists {
    ($path:expr) => {
        log_info!("File \"{}\" already exists.", $path);
    };
}

#[macro_export]
macro_rules! file_create_error {
    ($path:expr, $e:expr) => {
        log_err!("Error creating file '{}': {}", $path, $e);
    };
}

#[macro_export]
macro_rules! file_create_success {
    ($path:expr) => {
        log_ok!("File \"{}\" succesfully created", $path);
    };
}

#[macro_export]
macro_rules! file_read_error {
    ($path:expr, $e:expr) => {
        log_err!("Error reading file '{}': {}", $path, $e);
    };
}
