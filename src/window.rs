use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::KeyboardAndMouseInput::*,
    Windows::Win32::MenusAndResources::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsAndMessaging::*,
};

// Global instance required for the keyboard hook to get the window :(
static mut INSTANCE: *mut Window = std::ptr::null_mut();

#[derive(Debug)]
pub struct Window {
    handle: HWND,
    keyboard_hook: HHOOK,
}

impl Window {
    const WINDOW_CLASS_NAME: PSTR = PSTR(b"rl-audio-device-hotkey\0".as_ptr() as _);

    pub fn new() -> Box<Self> {
        let mut window = Box::new(Self {
            handle: HWND(0),
            keyboard_hook: HHOOK(0),
        });

        window.init();

        unsafe {
            assert!(
                INSTANCE == std::ptr::null_mut(),
                "Global Window has already been initialized!"
            );
            INSTANCE = std::mem::transmute(window);

            // Get that instance back out of INSTANCE so that we can return it.
            std::mem::transmute(INSTANCE)
        }
    }

    fn init(&mut self) {
        self.keyboard_hook = unsafe {
            SetWindowsHookExA(
                SetWindowsHookEx_idHook::WH_KEYBOARD_LL,
                Some(Self::keyboard_proc),
                HINSTANCE(0),
                0,
            )
        };
        debug_assert!(
            self.keyboard_hook != HHOOK::default(),
            "Keyboard hook could not be installed. GetLastError: {}",
            unsafe { GetLastError() }
        );

        let instance = HINSTANCE(unsafe { GetModuleHandleA(PSTR::default()) });
        debug_assert!(
            instance.0 != 0,
            "instance was invalid. GetLastError: {}",
            unsafe { GetLastError() }
        );

        let wc = WNDCLASSA {
            hInstance: instance,
            lpszClassName: Window::WINDOW_CLASS_NAME,
            lpfnWndProc: Some(Self::wndproc),
            ..Default::default()
        };

        let window_class = unsafe { RegisterClassA(&wc) };
        debug_assert!(
            window_class != 0,
            "window_class was invalid. GetLastError: {}",
            unsafe { GetLastError() }
        );

        let window = unsafe {
            CreateWindowExA(
                Default::default(),
                Window::WINDOW_CLASS_NAME,
                PSTR::default(),
                Default::default(),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND(0),
                HMENU(0),
                instance,
                self as *mut _ as _,
            )
        };
        debug_assert!(
            window.0 != 0,
            "window was invalid. GetLastError: {}",
            unsafe { GetLastError() }
        );
        debug_assert!(window == self.handle);
    }

    pub fn run(&mut self) {
        let mut message = MSG::default();

        loop {
            unsafe {
                GetMessageA(&mut message, HWND(0), 0, 0);
                if message.message == WM_QUIT {
                    println!("WM_QUIT");
                    return;
                }

                DispatchMessageA(&message);
            }
        }
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcA(self.handle, message, wparam, lparam) },
        }
    }

    fn keyboard_proc_handler(&mut self, code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code >= 0 {
            let p = lparam.0 as *const KBDLLHOOKSTRUCT;
            let p = unsafe { *p };
            let key_down = 0 == (p.flags & 0x80/*LLKHF_UP*/);

            if key_down {
                let ctrl_pressed = unsafe { GetKeyState(VK_CONTROL as _) } < 0;
                if ctrl_pressed && p.vkCode == VK_F12 {
                    //NextAudioPlaybackDevice();
                    println!("TADA!!");
                }

                println!(
                    "vkCode({}) scanCode({}) ctrl({})",
                    p.vkCode, p.scanCode, ctrl_pressed,
                );
                if p.vkCode == VK_ESCAPE {
                    unsafe {
                        PostMessageA(self.handle, WM_CLOSE, WPARAM(0), LPARAM(0));
                    }
                }
            }
        }

        unsafe { CallNextHookEx(self.keyboard_hook, code, wparam, lparam) }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            println!("Message: {}", message);

            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA, this as _);
            } else {
                let this =
                    GetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }

    extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let window = unsafe {
            assert!(
                INSTANCE != std::ptr::null_mut(),
                "Global Window is not initialized!"
            );
            &mut (*INSTANCE)
        };

        (*window).keyboard_proc_handler(code, wparam, lparam)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("Dropping!");

        unsafe {
            INSTANCE = std::ptr::null_mut();
        }

        unsafe {
            UnhookWindowsHookEx(self.keyboard_hook);
        }
    }
}
