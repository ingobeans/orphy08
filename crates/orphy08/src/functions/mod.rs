//! All the SDL2 functions that will need to be implemented
#![allow(clippy::missing_safety_doc)]

mod skeletons;
use std::{
    ffi::CStr,
    io::{Write, stdout},
    ptr::null_mut,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use enum_iterator::cardinality;

use crate::{
    BUFFERED_ENV_KEY, OrphyMessage, STDOUT_MESSAGE_HEADER, VIDEO_DATA_SIZE, VIDEO_MEMORY_LABEL,
    access_shared_memory, types::*,
};

static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);
const WINDOW_HANDLE: usize = 1;
static KEY_MAP: [u8; cardinality::<SDL_Scancode>()] = [0; _];
static SCREEN_BUFFER: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static SHARED_VIDEO_MEMORY: Mutex<SendPointer<u8>> = Mutex::new(SendPointer(null_mut()));
static IS_BUFFERED_RENDERING: AtomicBool = AtomicBool::new(false);
static mut SURFACE: SDL_Surface = SDL_Surface::new(null_mut());

struct SendPointer<T>(*mut T);
unsafe impl<T> Send for SendPointer<T> {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UpdateWindowSurface() -> i32 {
    if IS_BUFFERED_RENDERING.load(Ordering::Relaxed) {
        // If buffered rendering, copy video data to shared video memory
        unsafe {
            let lock = SCREEN_BUFFER.lock().unwrap();
            let screen_buffer = lock.as_ref().unwrap();
            assert_eq!(screen_buffer.len(), VIDEO_DATA_SIZE);

            std::ptr::copy_nonoverlapping(
                screen_buffer.as_ptr(),
                SHARED_VIDEO_MEMORY.lock().unwrap().0,
                VIDEO_DATA_SIZE,
            );
        }
    }

    let mut out = stdout();
    out.write_all(STDOUT_MESSAGE_HEADER).unwrap();
    out.write_all(&[OrphyMessage::NewFrame.to_index()]).unwrap();
    out.write_all(b"\n").unwrap();

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Init() -> i32 {
    let (_fd, addr) = access_shared_memory(VIDEO_MEMORY_LABEL, VIDEO_DATA_SIZE, false);
    *SHARED_VIDEO_MEMORY.lock().unwrap() = SendPointer(addr as _);

    // Check if rendering should be buffered
    let buffered = std::env::var(BUFFERED_ENV_KEY).is_ok_and(|a| a.as_str() == "true");
    IS_BUFFERED_RENDERING.store(buffered, Ordering::Relaxed);

    unsafe {
        if buffered {
            // If buffered, let Pico-8 write to SCREEN_BUFFER,
            // and only when SDL_UpdateWindowSurface is the data from there copied to
            // the shared video memory.
            let mut screen_buffer = SCREEN_BUFFER.lock().unwrap();
            *screen_buffer = Some(vec![0; VIDEO_DATA_SIZE]);
            SURFACE.pixels = screen_buffer.as_ref().unwrap().as_ptr();
        } else {
            SURFACE.pixels = addr as _;
        }
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
