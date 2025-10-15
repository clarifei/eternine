use anyhow::Result;

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

    loop {

        if input_simulator.is_xbutton2_pressed() {
            if screen_capture.capture_center_region().await
                .ok()
                .and_then(|frame| color_detector.detect_color(&frame).ok())
                .flatten()
                .is_some()
            {

                let _ = input_simulator.update_target_window();
                let _ = input_simulator.send_key_y();
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}