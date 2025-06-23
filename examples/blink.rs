//! Example: Blink the top-left LED on the Sense Hat matrix
//!
//! Run this example with:
//!     cargo run --example blink

use sensehat_rs::LedMatrix;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Try top open the framebuffer for the Sense HAT
    let mut matrix = LedMatrix::open().expect("Sense HAT Framebuffer not found!");

    // Blink the pixel (0, 0) red, 5 times
    for _ in 0..5 {
        matrix.set_pixel(0, 0, 255, 0, 0).unwrap();
        sleep(Duration::from_millis(750));
        matrix.set_pixel(0, 0, 0, 0, 0).unwrap();
        sleep(Duration::from_millis(750));
    }

    // Finally, clear the matrix (all LEDs off)
    matrix.clear().unwrap();
}
