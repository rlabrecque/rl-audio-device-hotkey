use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::MenusAndResources::*,
    Windows::Win32::SystemServices::*, Windows::Win32::WindowsAndMessaging::*,
};

pub struct Window {
    handle: HWND,
}

impl Window {
    const WINDOW_CLASS_NAME: PSTR = PSTR(b"rl-audio-device-hotkey\0".as_ptr() as _);

    pub fn new() -> Self {
        let mut window = Self { handle: HWND(0) };

        window.init();

        window
    }

    fn init(&mut self) {
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

    pub fn run(&self) {
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
}
