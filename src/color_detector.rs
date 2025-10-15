use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb};

pub struct ColorDetector {
    target_color: Rgb<u8>,
}

impl ColorDetector {

    pub fn new(hex_color: &str) -> Result<Self> {
        let target_color = parse_hex_color(hex_color)?;
        Ok(ColorDetector { target_color })
    }

    pub fn detect_color(&self, frame: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Option<()>> {

        let data = frame.as_raw();
        let [target_r, target_g, target_b] = self.target_color.0;

        for i in (0..data.len()).step_by(3) {
            let r = data[i];
            let g = data[i + 1];
            let b = data[i + 2];

            if r.abs_diff(target_r) <= 100 &&
               g.abs_diff(target_g) <= 100 &&
               b.abs_diff(target_b) <= 100 {
                return Ok(Some(()));
            }
        }

        Ok(None)
    }
}

fn parse_hex_color(hex_color: &str) -> Result<Rgb<u8>> {
    let hex = hex_color.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(anyhow!("Invalid hex color format: {}", hex_color));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| anyhow!("Invalid hex color: {}", hex_color))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| anyhow!("Invalid hex color: {}", hex_color))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| anyhow!("Invalid hex color: {}", hex_color))?;

    Ok(Rgb([r, g, b]))
}