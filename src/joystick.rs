
use evdev::{ Device, KeyCode, EventSummary };

/// A handler for the Raspberry Pi Sense HAT joystick.
///
/// Encapsulates access to the joystick Linux input device
/// and tracks the current cursor position (0..7).
/// Supports movement and the central Enter button.
/// Movement mode (Wrap or Clamp) is configurable.
///
/// # Example
/// ```
/// let mut joy = Joystick::open("/dev/input/event4", MovementMode::Clamp).unwrap();
/// joy.run(|x, y, evt| match evt {
///     JoystickEvent::Move  => println!("Moved to {}, {}", x, y),
///     JoystickEvent::Enter => println!("ENTER at {}, {}", x, y),
/// });
/// ```
pub struct Joystick {
    device: Device,
    x: usize,
    y: usize,
    mode: MovementMode,
}

/// Determines cursor behavior at the edge of the LED matrix:
/// - `Wrap`: wrap around to the opposite edge (like Snake/Pac-Man)
/// - `Clamp`: stop at the edge (classic behavior)
pub enum MovementMode {
    Wrap,  // current default
    Clamp, // stop at the edge of the matrix
}

/// All supported joystick events.
/// - `Move`: Cursor was moved.
/// - `Enter`: Central button was pressed.
/// (Planned: Release, LongPress, etc.)
pub enum JoystickEvent {
    Move,
    Enter,      // Joystick pressed
    // Release,
    // LongPress,
}

impl Joystick {
    /// Opens the joystick device at the given Linux device path (e.g. `/dev/input/event4`).
    /// The cursor starts at position (0, 0).
    ///
    /// Returns a new `Joystick` instance on success, or `None` ifthe device cannot be opened.
    ///
    /// # Example
    /// ```
    /// let joy = Joystick::open("/dev/input/event4", MovementMode::Wrap);
    /// ```
    pub fn open(dev_path: &str, mode: MovementMode) -> Option<Self> {
        Device::open(dev_path).ok().map(|dev| Self { device: dev, x: 0, y: 0, mode })
    }

    /// Returns the current (x, y) position of the joystick cursor.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Starts the main event loop for the joystick.
    ///
    /// The provided callback is invoked whenever the cursor position changes
    /// (on movement), or the Enter button is pressed.
    /// The callbac receives the new coordinates and the event type (`Move` or `Enter`).
    ///
    /// This loop is blocking and will only return if the program is terminated.
    /// For custom exit logic, handle it in your callback.
    ///
    /// # Example
    /// ```
    /// joy.run(|x, y, evt| match evt {
    ///     JoystickEvent::Move  => println!("Moved: ({}, {})", x, y),
    ///     JoystickEvent::Enter => println!("Enter pressed at ({}, {})", x, y),
    /// });
    /// ```
    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(usize, usize, JoystickEvent), //The callback receives the current x and y as arguments
    {
        loop {
            // Fetch all new events from the device
            for ev in self.device.fetch_events().unwrap() {
                match ev.destructure() {
                    // Only react to KEY events (joystick buttons)
                    EventSummary::Key(_, keycode, value) => {
                        // value == 1 -> button pressed (not released!)
                        if value == 1 {
                            let mut moved: bool = false;
                            let mut event = None;

                            // Movement logic based on key and movement mode
                            match keycode {
                                // Down:
                                KeyCode::KEY_DOWN => {
                                    match self.mode {
                                        MovementMode::Wrap => { self.y = if self.y < 7 { self.y + 1 } else { 0 }; }
                                        MovementMode::Clamp => { if self.y < 7 { self.y += 1; } }                                        
                                    }
                                    moved = true;
                                }
                                // Up:
                                KeyCode::KEY_UP => {
                                    match self.mode {
                                        MovementMode::Wrap => { self.y = if self.y > 0 { self.y - 1 } else { 7 }; }
                                        MovementMode::Clamp => { if self.y > 0 { self.y -= 1; } }
                                    }
                                    moved = true;
                                }
                                // Right:
                                KeyCode::KEY_RIGHT => {
                                    match self.mode {
                                        MovementMode::Wrap => { self.x = if self.x < 7 { self.x + 1 } else { 0 }; }
                                        MovementMode::Clamp => { if self.x < 7 { self.x += 1; } }
                                    }
                                    moved = true;
                                }
                                // Left:
                                KeyCode::KEY_LEFT => {
                                    match self.mode {
                                        MovementMode::Wrap => { self.x = if self.x > 0 { self.x - 1} else { 7 }; }
                                        MovementMode::Clamp => { if self.x > 0 { self.x -= 1; } }
                                    }
                                    moved = true;
                                }
                                // Enter (center button)
                                KeyCode::KEY_ENTER => {
                                    event = Some(JoystickEvent::Enter);
                                }
                                _ => {}
                            }

                            // Fire callback for movement or special event
                            if moved {
                                callback(self.x, self.y, JoystickEvent::Move);
                            } else if let Some(evt) = event {
                                callback(self.x, self.y, evt);
                            }
                        }
                    }
                    // Ignore all other event types
                    _ => {}
                }
            }
        }
    }
}
