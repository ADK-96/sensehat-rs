//! Library root for sensehat-rs.  
//! Re-exports LED matrix and joystick functionality.

pub mod led;
pub mod joystick;

pub use led::LedMatrix;
pub use joystick::{ Joystick, MovementMode, JoystickEvent };