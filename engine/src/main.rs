//pub mod display;
pub mod editor;
pub mod errors;
pub mod gpu;
pub mod window;

fn main() {
    env_logger::init();

    window::run();
}
