#![windows_subsystem = "windows"]

mod keyboard_hook;
mod window;

use bindings::{
    Windows::Win32::KeyboardAndMouseInput::GetKeyState,
    Windows::Win32::WindowsAndMessaging::{
        KBDLLHOOKSTRUCT, VK_CONTROL, VK_F12,
    },
};
use keyboard_hook::KeyboardHook;
use window::Window;

fn main() {
    let window = Window::new();
    let _keyboard_hook = KeyboardHook::new(keyboard_proc_handler);
    window.run();
}

fn keyboard_proc_handler(data: &KBDLLHOOKSTRUCT) -> bool {
    const LLKHF_UP: u32 = 0x80;
    let key_down = (data.flags & LLKHF_UP) != LLKHF_UP;

    if key_down {
        let ctrl_pressed = unsafe { GetKeyState(VK_CONTROL as _) } < 0;
        if ctrl_pressed && data.vkCode == VK_F12 {
            //switch_to_next_audio_device();
            println!("TADA!!");
            return true;
        }

        println!(
            "vkCode({}) scanCode({}) ctrl({})",
            data.vkCode, data.scanCode, ctrl_pressed,
        );
    }

    false
}
