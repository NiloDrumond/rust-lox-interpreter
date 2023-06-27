pub fn report_error(line: usize, location: &str, message: &str) {
    eprintln!("[line {line}] Error{location}: {message}");
}
