fn main() {
    windows::build!(
        Windows::Win32::Debug::GetLastError,
        Windows::Win32::KeyboardAndMouseInput::GetKeyState,
        Windows::Win32::MenusAndResources::HMENU,
        Windows::Win32::SystemServices::{GetModuleHandleA, HINSTANCE, LRESULT, PSTR},
        Windows::Win32::WindowsAndMessaging::{
            CallNextHookEx, CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA,
            GetMessageA, GetWindowLongPtrA, LoadIconA, PostMessageA, PostQuitMessage, RegisterClassA,
            SetWindowLongPtrA, SetWindowsHookExA, SetWindowsHookEx_idHook, UnhookWindowsHookEx,
            CREATESTRUCTA, CW_USEDEFAULT, HHOOK, HOOKPROC, HWND, KBDLLHOOKSTRUCT, LPARAM, MSG,
            VK_CONTROL, VK_ESCAPE, VK_F12, WINDOW_LONG_PTR_INDEX, WM_CLOSE, WM_DESTROY, WM_NCCREATE,
            WM_QUIT, WNDCLASSA, WPARAM, WM_KEYUP, WM_SYSKEYUP,
        },
       );
}
