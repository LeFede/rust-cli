#[macro_export]
macro_rules! log_err {
    ($fmt:literal, $($arg:tt)*) => {
        eprintln!("❌ {}", format!($fmt, $($arg)*).red());
    };
}
