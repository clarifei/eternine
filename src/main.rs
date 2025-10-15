use anyhow::Result;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

mod color_detector;
mod screen_capture;
mod input_simulator;

use color_detector::ColorDetector;
use screen_capture::ScreenCapture;
use input_simulator::InputSimulator;


#[tokio::main]
async fn main() -> Result<()> {
    let screen_capture = ScreenCapture::new()?;
    let color_detector = ColorDetector::new("#EE0201")?;
    let mut input_simulator = InputSimulator::new()?;

    let input_delay = Arc::new(AtomicU32::new(0));
    let input_delay_clone = input_delay.clone();
    
    tokio::spawn(async move {
        check_hotkeys(input_delay_clone).await;
    });

    loop {
        if input_simulator.is_xbutton2_pressed() {
            if screen_capture.capture_center_region().await
                .ok()
                .and_then(|frame| color_detector.detect_color(&frame).ok())
                .flatten()
                .is_some()
            {
                let input_delay_ms = input_delay.load(Ordering::Relaxed);
                if input_delay_ms > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(input_delay_ms as u64)).await;
                }

                let _ = input_simulator.update_target_window();
                let _ = input_simulator.send_key_y();
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }
}

async fn check_hotkeys(input_delay: Arc<AtomicU32>) {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_SHIFT, VK_UP, VK_DOWN
    };

    loop {
        unsafe {
            let ctrl_pressed = GetAsyncKeyState(VK_CONTROL.0 as i32) < 0;
            let shift_pressed = GetAsyncKeyState(VK_SHIFT.0 as i32) < 0;
            let up_pressed = GetAsyncKeyState(VK_UP.0 as i32) < 0;
            let down_pressed = GetAsyncKeyState(VK_DOWN.0 as i32) < 0;
            
            if ctrl_pressed {
                let increment = if shift_pressed { 50 } else { 10 };
                
                if up_pressed {
                    let current = input_delay.load(Ordering::Relaxed);
                    input_delay.store(current + increment, Ordering::Relaxed);
                    println!("Input delay increased to {}ms", current + increment);
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                } else if down_pressed {
                    let current = input_delay.load(Ordering::Relaxed);
                    let new_delay = if current >= increment { current - increment } else { 0 };
                    input_delay.store(new_delay, Ordering::Relaxed);
                    println!("Input delay decreased to {}ms", new_delay);
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                }
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
}