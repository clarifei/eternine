use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb};
use xcap::Monitor;

pub struct ScreenCapture {
    monitor: Monitor,
    width: u32,
    height: u32,
    region_size: (u32, u32), 
}

impl ScreenCapture {

    pub fn new() -> Result<Self> {

        let monitors = Monitor::all()?;
        if monitors.is_empty() {
            return Err(anyhow!("No monitors found"));
        }

        let monitor = monitors.into_iter().next().unwrap();

        let region_size = (4, 4);

        let width = monitor.width()?;
        let height = monitor.height()?;

        Ok(ScreenCapture {
            monitor,
            width,
            height,
            region_size,
        })
    }

    pub async fn capture_center_region(&self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {

        let center_x = (self.width - self.region_size.0) >> 1; 
        let center_y = (self.height - self.region_size.1) >> 1; 

        self.capture_region(center_x, center_y, self.region_size.0, self.region_size.1)
    }

    fn capture_region(&self, x: u32, y: u32, width: u32, height: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {

        let image = self.monitor.capture_region(x, y, width, height)?;

        let rgb_data = self.bgra_to_rgb(&image, width, height)?;

        Ok(ImageBuffer::from_raw(width, height, rgb_data)
            .ok_or_else(|| anyhow!("Failed to create image buffer"))?)
    }

    fn bgra_to_rgb(&self, bgra_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        let pixel_count = (width * height) as usize;
        let mut rgb_data = Vec::with_capacity(pixel_count * 3);

        unsafe {
            rgb_data.set_len(pixel_count * 3);
        }

        let mut rgb_index = 0;
        for chunk in bgra_data.chunks_exact(4) {
            unsafe {
                *rgb_data.get_unchecked_mut(rgb_index) = *chunk.get_unchecked(0);
                *rgb_data.get_unchecked_mut(rgb_index + 1) = *chunk.get_unchecked(1);
                *rgb_data.get_unchecked_mut(rgb_index + 2) = *chunk.get_unchecked(2);
            }
            rgb_index += 3;
        }

        Ok(rgb_data)
    }

}