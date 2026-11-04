#![allow(clippy::missing_safety_doc)]

use std::{
    process::{self, Stdio},
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
};

use macroquad::{miniquad::window::screen_size, prelude::*};
use orphy08::*;

fn window() -> Conf {
    Conf {
        window_title: "orphy08".to_string(),
        window_width: 128 * 5,
        window_height: 128 * 5,
        ..Default::default()
    }
}

static NEW_FRAME_PIXELS: Mutex<[u8; VIDEO_DATA_SIZE]> = Mutex::new([0; _]);
static NEW_FRAME: AtomicBool = AtomicBool::new(false);

fn draw_pixel_buffer() {
    // *NEW_FRAME_PIXELS.lock().unwrap() = unsafe { PIXELS_BUFFER };
    *NEW_FRAME_PIXELS.lock().unwrap() = [0; _];
    NEW_FRAME.store(true, Ordering::Relaxed);
}

/// Swaps the Red and Blue color channels in a pixel buffer
fn swap_red_blue(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }
}

#[macroquad::main(window)]
async fn main() {
    let (_fd, addr) = access_shared_memory(VIDEO_MEMORY_LABEL, VIDEO_DATA_SIZE, true);
    // let thread = launch_orphy08();

    let texture = Texture2D::from_rgba8(128, 128, &[0; VIDEO_DATA_SIZE]);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // draw new frame if it is available
        if true || NEW_FRAME.swap(false, Ordering::Relaxed) {
            // let mut pixels = NEW_FRAME_PIXELS.lock().unwrap();
            let mut pixels = unsafe {
                let pixels = std::slice::from_raw_parts(addr as *mut u8, VIDEO_DATA_SIZE);
                pixels.to_vec()
            };

            // Pico-8 uses the BGRA color format, while macroquad requires RGBA
            swap_red_blue(&mut pixels);

            texture.update_from_bytes(128, 128, &pixels[..]);
        }

        let (mut screen_w, mut screen_h) = screen_size();
        let scale = screen_dpi_scale();
        screen_w /= scale;
        screen_h /= scale;
        let scale_factor = (screen_w / 128.0).min(screen_h / 128.0);
        let size = 128.0 * scale_factor;
        let x = (screen_w - size) / 2.0;
        let y = (screen_h - size) / 2.0;

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(size, size)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
