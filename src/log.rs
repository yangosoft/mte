use std::env;
pub fn Log(text: &str) {
    if env::var("TRACE_ENABLED").is_ok() {
        eprintln!("{}", text);
    }
}
