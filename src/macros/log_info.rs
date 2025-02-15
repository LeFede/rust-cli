#[macro_export]
macro_rules! log_info {
    ($fmt:literal, $($arg:tt)*) => {
        println!("ℹ️  {}", format!($fmt, $($arg)*).blue());
    };
}
