
use std::fs::OpenOptions;
use std::io::{ Seek, SeekFrom, Write, Result };



/// A struct for controlling the Raspberry Pi Sense HAT LED matrix via the framebuffer device.
///
/// Provides basic methods to set individual pixels and clear the display.
/// Assumes the device is available at `/dev/fb1` (standard for Sense HAT).
pub struct LedMatrix {
    fb: std::fs::File,
}

impl LedMatrix {
    /// Opens the Sense HAT framebuffer device (`/dev/fb1` by default).
    ///
    /// # Returns
    /// * `Ok(LedMatrix)` if successful.
    /// * `Err` if the device cannot be opened.
    pub fn open() -> Result<Self> {
        let fb = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0")?; // <- could be "/dev/fb1" -> Change to your specific path!
        Ok(LedMatrix {fb})
    }

    /// Sets a single pixel at `(x, y)` to the specified RGB color.
    ///
    /// # Arguments
    /// * `x`, `y` - Pixel coordinates (0..7)
    /// * `r`, `g`, `b` - Color values (0..255)
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err` on I/O error.
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> Result<()> {
        let offset = (y * 8 + x) * 2;
        let data = rgb_to_rgb565(r, g, b);
        self.fb.seek(SeekFrom::Start(offset as u64))?;
        self.fb.write_all(&data)?;
        Ok(())
    }

    /// Clears the entire LED matrix (sets all pixels to black/off).
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err` on I/O error.
    pub fn clear(&mut self) -> Result<()> {
        // For 8x8, RGB565: 8*8*2 = 128 bytes
        let zeros = [0u8; 128];
        self.fb.seek(SeekFrom::Start(0))?;
        self.fb.write_all(&zeros)?;
        Ok(())
    }

}

/// Helper: Converts 8-bit RGB to 16-bit RGB565 color format for the framebuffer.
///
/// # Arguments
/// * `r`, `g`, `b` - Color values (0..255)
///
/// # Returns
/// * `[u8; 2]` - Two-byte little-endian value in RGB565 format.
fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> [u8; 2] {
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;
    let value = (r5 << 11) | (g6 << 5) | b5;
    value.to_le_bytes()
}
