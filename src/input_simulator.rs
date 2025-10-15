use anyhow::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, WM_KEYDOWN, WM_KEYUP,
};
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_XBUTTON2, VK_Y,
};

pub struct InputSimulator {
    target_window: HWND,
}

impl InputSimulator {
    pub fn new() -> Result<Self> {

        let target_window = unsafe {
            windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow()
        };

        Ok(InputSimulator { target_window })
    }

    pub fn is_xbutton2_pressed(&self) -> bool {
        unsafe {
            GetAsyncKeyState(VK_XBUTTON2.0 as i32) < 0
        }
    }

    pub fn send_key_y(&self) -> Result<()> {
        unsafe {

            let _ = PostMessageW(
                Some(self.target_window),
                WM_KEYDOWN,
                WPARAM(VK_Y.0 as usize),
                LPARAM(0),
            );

            let _ = PostMessageW(
                Some(self.target_window),
                WM_KEYUP,
                WPARAM(VK_Y.0 as usize),
                LPARAM(0),
            );
        }

        Ok(())
    }

    pub fn update_target_window(&mut self) -> Result<()> {
        self.target_window = unsafe {
            windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow()
        };
        Ok(())
    }
}