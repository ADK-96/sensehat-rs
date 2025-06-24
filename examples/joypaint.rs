use sensehat_rs::{LedMatrix, Joystick, MovementMode, JoystickEvent};

/// Interactive LED painter using the Sense HAT joystick and LED matrix.
/// Move the cursor with the joystick, toggle LEDs on/off with Enter.
/// Cursor is shown in blue, painted LEDs in green.
fn main() {
    println!("Joystick-LED-Painter started!");

    // Open the LED matrix device
    let mut matrix = LedMatrix::open().unwrap();

    // Open the joystick device (adjust device path if needed)
    let mut joy = Joystick::open("/dev/input/event4", MovementMode::Wrap).unwrap();

    // Paint buffer: 8x8 grid, tracks which LEDs are toggled on
    let mut painted = [[false; 8]; 8];

    // Draw the initial cursor position
    let (mut x, mut y) = joy.position();
    matrix.clear().unwrap();
    matrix.set_pixel(x, y, 0, 0, 255).unwrap(); // Blue cursor

    // Start the joystick event loop
    joy.run(|new_x, new_y, evt| match evt {
        JoystickEvent::Move => {
            // Update cursor position
            x = new_x;
            y = new_y;
            // Clear matrix before redrawing
            matrix.clear().unwrap();

            // Redraw all painted LEDs (green)
            for (py, row) in painted.iter().enumerate() {
                for (px, &on) in row.iter().enumerate() {
                    if on {
                        matrix.set_pixel(px, py, 0, 255, 0).unwrap(); // Green: painted
                    }
                }
            }
            // Draw the blue cursor at the new position
            matrix.set_pixel(x, y, 0, 0, 255).unwrap();
        }
        JoystickEvent::Enter => {
            // Toggle the painted state at the current position
            painted[y][x] = !painted[y][x];

            // Set LED color accordingly (green = on, black = off)
            if painted[y][x] {
                matrix.set_pixel(x, y, 0, 255, 0).unwrap(); // Green: on
            } else {
                matrix.set_pixel(x, y, 0, 0, 0).unwrap();   // Black: off
            }
        }
    });
}
