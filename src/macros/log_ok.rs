#[macro_export]
macro_rules! log_ok {
    ($fmt:literal, $($arg:tt)*) => {
        println!("✅ {}", format!($fmt, $($arg)*).green());
    };
}
