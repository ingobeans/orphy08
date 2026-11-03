mod sdl;
use std::{sync::Mutex, time::Instant};

pub const SCREEN_DATA_SIZE: usize = 128 * 128 * 4;

pub static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);
pub static mut PIXELS_BUFFER: [u8; SCREEN_DATA_SIZE] = [0; _];

/// Library specific function.
/// Must be called at initialization
pub fn initialize() {
    *START_TIME.lock().unwrap() = Some(Instant::now());
}
