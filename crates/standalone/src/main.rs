#![allow(clippy::missing_safety_doc)]

use macroquad::{miniquad::window::screen_size, prelude::*};
use orphy08::{debug::write_char_to_buffer, *};

fn window() -> Conf {
    Conf {
        window_title: "orphy08".to_string(),
        window_width: 128 * 5,
        window_height: 128 * 5,
        ..Default::default()
    }
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
    set_up_memory_cleanup();
    let mut d = [0_u8; VIDEO_DATA_SIZE];
    let addr = d.as_mut_ptr();

    let cart = std::env::args().nth(1).unwrap_or_default();
    let rx = launch_orphy08(&cart, true);

    let texture = Texture2D::from_rgba8(128, 128, &[0; VIDEO_DATA_SIZE]);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // Draw new frame if it is available
        if let Ok(OrphyMessage::NewFrame) = rx.try_recv() {
            let mut pixels = unsafe {
                let pixels = std::slice::from_raw_parts(addr as *mut u8, VIDEO_DATA_SIZE);
                pixels.to_vec()
            };

            // Pico-8 uses the BGRA color format, while macroquad requires RGBA
            swap_red_blue(&mut pixels);

            unsafe {
                let slice: &mut [[u8; 4]] =
                    std::slice::from_raw_parts_mut(addr as _, VIDEO_DATA_SIZE / 4);
                slice.fill([0, 0, 0, 0]);
                write_char_to_buffer(
                    'F' as _,
                    slice,
                    128,
                    64,
                    64,
                    [255, 255, 255, 255],
                    Some([0, 0, 0, 255]),
                );
            }

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
