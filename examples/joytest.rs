use sensehat_rs ::{ LedMatrix, Joystick, MovementMode, JoystickEvent };

/// Example usage of the Sense HAT joystick and LED matrix.
/// Shows basic event-driven handling with clean output to the console.
fn main() {
    println!("Program started...");

    // Open the LED matrix device.
    let mut matrix = LedMatrix::open().unwrap();
    println!("LedMatrix opened!");

    // Open the joystick on the specified Linux device path with clamp movement mode.
    let mut joy = Joystick::open("/dev/input/event4", MovementMode::Clamp).unwrap();
    println!("Joystick opened!");

    // Clear the matrix before starting.
    matrix.clear().unwrap();
    println!("Matrix cleared!");

    // Start the joystick event loop.
    // For every movement or Enter button press, call the callback.
    joy.run(|x, y, evt| match evt {
        JoystickEvent::Move => {
            // Print new position and update the display
            println!("Moved to {}, {}", x, y);
            matrix.clear().unwrap();
            matrix.set_pixel(x, y, 0, 0, 255).unwrap(); // Set new pixel blue
        }
        JoystickEvent::Enter => {
            // Print Enter press info
            println!("ENTER at {}, {}", x, y);
            // (You can add more fancy behavior her if needed)
        }
    });
}
