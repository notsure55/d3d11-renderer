use anyhow::Result;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

pub struct Window {
    pub instance: HINSTANCE,
    pub hwnd: HWND,
}

impl Window {
    pub fn new() -> Result<Self> {
        unsafe {
            let instance: HINSTANCE = GetModuleHandleA(None)?.into();
            debug_assert!(!instance.0.is_null());

            let window_class = w!("window");

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExW(
                WS_EX_OVERLAPPEDWINDOW,
                window_class,
                w!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                Some(instance),
                None,
            )?;

            Ok(Self { instance, hwnd })
        }
    }
    pub fn process_msg(&self) -> Result<()> {
        unsafe {
            let mut msg = MSG::default();

            let got_msg = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) != false;

            if got_msg == true {
                // Translate and dispatch the message
                TranslateMessage(&msg);
                let _ = DispatchMessageW(&msg);

                if msg.message == WM_QUIT {
                    return Err(anyhow::anyhow!("Exiting!"));
                }
            }

            Ok(())
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(Some(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
