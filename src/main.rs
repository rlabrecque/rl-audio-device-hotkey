#![windows_subsystem = "windows"]

mod window;

use window::Window;

fn main() {
    let mut window = Window::new();
    window.run();
}
