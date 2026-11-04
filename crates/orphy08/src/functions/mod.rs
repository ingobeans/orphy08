//! All the SDL2 functions that will need to be implemented
#![allow(clippy::missing_safety_doc)]

mod skeletons;
use std::{
    ffi::CStr,
    io::{Write, stdout},
    ptr::null_mut,
    sync::Mutex,
    time::Instant,
};

use enum_iterator::cardinality;

use crate::{
    OrphyMessage, STDOUT_MESSAGE_HEADER, VIDEO_DATA_SIZE, VIDEO_MEMORY_LABEL, access_shared_memory,
    types::*,
};

pub static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);
pub const WINDOW_HANDLE: usize = 1;
pub static KEY_MAP: [u8; cardinality::<SDL_Scancode>()] = [0; _];
pub static mut SURFACE: SDL_Surface = SDL_Surface::new(null_mut());

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UpdateWindowSurface() -> i32 {
    let mut out = stdout();
    out.write_all(STDOUT_MESSAGE_HEADER).unwrap();
    out.write_all(&[OrphyMessage::NewFrame.to_index()]).unwrap();
    out.write_all(b"\n").unwrap();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Init() -> i32 {
    let (_fd, addr) = access_shared_memory(VIDEO_MEMORY_LABEL, VIDEO_DATA_SIZE, false);

    unsafe {
        SURFACE.pixels = addr as _;
    }

    *START_TIME.lock().unwrap() = Some(Instant::now());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetTicks() -> u64 {
    let now = Instant::now();

    let start = START_TIME.lock().unwrap().unwrap_or(now);

    (now - start).as_millis() as _
}

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
