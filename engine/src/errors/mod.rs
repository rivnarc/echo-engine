pub fn set_error(code: f32) {
    eprintln!(
        "\n\x1b[1;31m[ERROR {}]\x1b[0m For more information, see the 'error_list.rs' file.",
        code
    );
    std::process::exit(1);
}
