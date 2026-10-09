pub mod editor;
pub mod gpu;
pub mod logs;
pub mod window;

use logs::logger::info;

fn main() {
    info("Engine started", 0.0);
    window::run();
    info("Engine finished without any critical error", 0.1)
}
