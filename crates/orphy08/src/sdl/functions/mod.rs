//! All the SDL2 functions that will need to be implemented
#![allow(clippy::missing_safety_doc)]

mod skeletons;
use std::ffi::CStr;

use enum_iterator::cardinality;
pub use skeletons::*;

use crate::sdl::types::*;

pub const WINDOW_HANDLE: usize = 1;
pub static KEY_MAP: [u8; cardinality::<SDL_Scancode>()] = [0; _];
pub static mut PIXELS: [u8; 128 * 128 * 4] = [0; _];
pub static mut SURFACE: SDL_Surface = SDL_Surface::new(&raw mut PIXELS as *mut _ as _);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetKeyboardState(out_len: *mut i32) -> *const u8 {
    unsafe {
        *out_len = KEY_MAP.len() as _;
    }
    KEY_MAP.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateTexture() -> u64 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowSurface(handle: usize) -> *mut SDL_Surface {
    assert_eq!(handle, WINDOW_HANDLE);
    &raw mut SURFACE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowSize(handle: usize, w: *mut i32, h: *mut i32) {
    assert_eq!(handle, WINDOW_HANDLE);
    unsafe {
        *w = 128;
        *h = 128;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateWindow(
    title: *const i8,
    _x: u64,
    _y: u64,
    w: u64,
    h: u64,
    flags: u64,
) -> usize {
    unsafe {
        println!(
            "Creating window: {:?} ({w}x{h}) with flags: 0x{flags:x}. Addr: 0x{:x}",
            CStr::from_ptr(title),
            WINDOW_HANDLE
        );
    }
    WINDOW_HANDLE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DestroyWindow(handle: usize) {
    assert_eq!(handle, WINDOW_HANDLE);
    println!("Destroying window: 0x{handle:x}");
}
