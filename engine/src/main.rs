use log::info;

pub mod display;
pub mod errors;
pub mod gpu_context;
pub mod window;

fn main() {
    env_logger::init();
    let displays: Vec<display::Display> = display::check_displays();
    let main_display: &display::Display = display::get_primary_display(&displays);
    info!(
        "Main display: {}x{} (id {})",
        main_display.width, main_display.height, main_display.id,
    );

    window::run();
}
