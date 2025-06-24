use sensehat_rs::LedMatrix;
use std::thread::sleep;
use std::time::Duration;

/// Pixel data for "HI" (8x8)
/// 1 = on, 0 = off
const HI: [[u8; 8]; 8] = [
    // H       I
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
];

fn main() {
    let mut matrix = LedMatrix::open().unwrap();

    // Clear matrix before displaying
    matrix.clear().unwrap();

    // Display "HI" in green
    for (y, row) in HI.iter().enumerate() {
        for (x, &on) in row.iter().enumerate() {
            if on == 1 {
                matrix.set_pixel(x, y, 0, 255, 0).unwrap();
            }
        }
    }

    // Wait for 5 seconds before clearing the matrix and exiting
    sleep(Duration::from_secs(5));
    matrix.clear().unwrap();
}
