//! All the SDL2 functions that will need to be implemented

mod skeletons;
use std::ffi::CStr;

pub use skeletons::*;

use crate::sdl::types::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateWindow(
    title: *const i8,
    _x: u64,
    _y: u64,
    w: u64,
    h: u64,
    flags: u64,
) -> *mut SDL_Window {
    let window = Box::into_raw(Box::new(SDL_Window(0)));
    unsafe {
        println!(
            "Creating window: {:?} ({w}x{h}) with flags: 0x{flags:x}. Addr: 0x{:x}",
            CStr::from_ptr(title),
            window as usize
        );
    }
    window
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DestroyWindow(ptr: *mut SDL_Window) {
    println!("Destroying window: 0x{:x}", ptr as usize);
    let b = unsafe { Box::from_raw(ptr) };
    drop(b);
}
