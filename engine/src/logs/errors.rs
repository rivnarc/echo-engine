pub fn set_error(code: f32) {
    let error_explication: &str = match code {
        0.0 => "Test error",
        0.1 => "Impossible to get default config from surface",
        _ => "Unknown error",
    };
    eprintln!("\x1b[1;31m[ERROR {}]:\x1b[0m {}.", code, error_explication);
    std::process::exit(1);
}
