pub fn info(log: &str, code: f32) {
    eprintln!("\x1b[1;34m[INFO {}]:\x1b[0m {}.", code, log);
}

pub fn warning(log: &str, code: f32) {
    eprintln!("\x1b[1;33m[WARNING {}]:\x1b[0m {}.", code, log);
}
