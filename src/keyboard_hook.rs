use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsAndMessaging::*,
};

type OnKeyDownCallback = fn(data: &KBDLLHOOKSTRUCT) -> bool;

// Global instance required for the keyboard hook to get the window :(
static mut INSTANCE: *mut KeyboardHook = std::ptr::null_mut();

pub struct KeyboardHook {
    hook: HHOOK,
    on_key_down_event: OnKeyDownCallback,
}

impl KeyboardHook {
    pub fn new(on_key_down_event: OnKeyDownCallback) -> Box<Self> {
        let mut keyboard_hook = Box::new(Self {
            hook: HHOOK(0),
            on_key_down_event,
        });

        keyboard_hook.init();

        unsafe {
            assert!(
                INSTANCE == std::ptr::null_mut(),
                "Global Window has already been initialized!"
            );
            INSTANCE = std::mem::transmute(keyboard_hook);

            // Get that instance back out of INSTANCE so that we can return it.
            std::mem::transmute(INSTANCE)
        }
    }

    fn init(&mut self) {
        let instance = HINSTANCE(unsafe { GetModuleHandleA(PSTR::default()) });
        debug_assert!(
            instance.0 != 0,
            "instance was invalid. GetLastError: {}",
            unsafe { GetLastError() }
        );

        self.hook = unsafe {
            SetWindowsHookExA(
                SetWindowsHookEx_idHook::WH_KEYBOARD_LL,
                Some(Self::keyboard_proc),
                instance,
                0,
            )
        };
        debug_assert!(
            self.hook != HHOOK::default(),
            "Keyboard hook could not be installed. GetLastError: {}",
            unsafe { GetLastError() }
        );
    }

    extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let keyboard_hook = unsafe {
            assert!(
                INSTANCE != std::ptr::null_mut(),
                "Global Window is not initialized!"
            );
            &mut (*INSTANCE)
        };

        if code >= 0 {
            let p = lparam.0 as *const KBDLLHOOKSTRUCT;
            let p = unsafe { *p };
            let handled = (keyboard_hook.on_key_down_event)(&p);
            if handled {
                return LRESULT(-1);
            }
        }

        unsafe { CallNextHookEx(keyboard_hook.hook, code, wparam, lparam) }
    }
}

impl Drop for KeyboardHook {
    fn drop(&mut self) {
        println!("Dropping!");

        unsafe {
            INSTANCE = std::ptr::null_mut();
        }

        unsafe {
            UnhookWindowsHookEx(self.hook);
        }
    }
}
