#![windows_subsystem = "windows"]

mod keyboard_hook;
mod audio_device;
mod window;

use bindings::{
    Windows::Win32::KeyboardAndMouseInput::GetKeyState,
    Windows::Win32::WindowsAndMessaging::{KBDLLHOOKSTRUCT, VK_CONTROL, VK_F12},
};
use keyboard_hook::KeyboardHook;
use window::Window;

fn main() {
    let window = Window::new();
    let _keyboard_hook = KeyboardHook::new(on_key_released_handler);
    window.run();
}

fn on_key_released_handler(data: &KBDLLHOOKSTRUCT) -> bool {
    let ctrl_pressed = unsafe { GetKeyState(VK_CONTROL as _) } < 0;
    if ctrl_pressed && data.vkCode == VK_F12 {
        println!("TADA!!");
        match audio_device::switch_to_next_output_device() {
            Ok(_) => return true,
            Err(err) => {
                debug_assert!(false, "switch_to_next_output_device failed: {}", err);
                return false;
            }
        };
    }

    println!(
        "vkCode({}) scanCode({}) ctrl({})",
        data.vkCode, data.scanCode, ctrl_pressed,
    );

    false
}
