use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write, Result};

/// Struct to control the Raspberry Pi Sense HAT LED matrix via framebuffer.
pub struct LedMatrix {
    fb: std::fs::File,
}

impl LedMatrix {
    /// Tries to open the Sense HAT framebuffer device ('/dev/fb0' is standard, change if needed)
    ///
    /// # Returns
    /// * `Ok(LedMatrix)` if successful.
    /// * `Err` if device cannot be opened.
    pub fn open() -> Result<Self> {
        let fb = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0")?; // default fb0
        Ok(LedMatrix { fb })
    }

    /// Sets a single pixel (x, y) to the specified RGB color.
    ///
    /// # Arguments
    /// * `x`, `y` - Pixel coordinates (0..7)
    /// * `r`, `g`, `b` - Color values (0...255)
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> Result<()> {
        let offset = (y * 8 + x) * 2;
        let data = rgb_to_rgb565(r, g, b);
        self.fb.seek(SeekFrom::Start(offset as u64))?;
        self.fb.write_all(&data)?;
        Ok(())
    }

    /// Clears the entire LED matrix (sets all pixels to off).
    pub fn clear(&mut self) -> Result<()> {
        for y in 0..8 {
            for x in 0..8 {
                self.set_pixel(x, y, 0, 0, 0)?;
            }
        }
        Ok(())
    }
}

/// Helper to encode RGB values to RGB565 (16-bit color format).
fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> [u8; 2] {
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;
    let value = (r5 << 11) | (g6 << 5) | b5;
    value.to_le_bytes()
}

