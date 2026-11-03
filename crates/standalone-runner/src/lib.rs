use std::{
    process::exit,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
};

use macroquad::{miniquad::window::screen_size, prelude::*};
pub use orphy08::*;

fn window() -> Conf {
    Conf {
        window_title: "orphy08".to_string(),
        window_width: 128 * 5,
        window_height: 128 * 5,
        ..Default::default()
    }
}

static NEW_FRAME_PIXELS: Mutex<[u8; SCREEN_DATA_SIZE]> = Mutex::new([0; _]);
static NEW_FRAME: AtomicBool = AtomicBool::new(false);
static WINDOW: AtomicBool = AtomicBool::new(false);
static WINDOW_THREAD: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UnlockSurface() {
    if !WINDOW.swap(true, Ordering::Relaxed) {
        let thread = run_window_thread();
        let mut lock = WINDOW_THREAD.lock().unwrap();
        *lock = Some(thread);
    } else {
        let mut lock = WINDOW_THREAD.lock().unwrap();
        let thread = lock.as_mut().unwrap();
        if thread.is_finished() {
            let thread = lock.take().unwrap();
            thread.join().unwrap();
            exit(0);
        }
    }
    *NEW_FRAME_PIXELS.lock().unwrap() = unsafe { PIXELS };
    NEW_FRAME.store(true, Ordering::Relaxed);
}
fn run_window_thread() -> JoinHandle<()> {
    std::thread::spawn(|| {
        main();
    })
}

fn analyze_screen_data(data: [u8; SCREEN_DATA_SIZE]) {
    for (i, p) in data.chunks_exact(4).enumerate() {
        if (p[0] + p[1] + p[2] > 0) || p[3] != 255 {
            println!("B: {p:?} at {i}");
        }
    }
}

#[macroquad::main(window)]
async fn main() {
    let texture = Texture2D::from_rgba8(128, 128, &[0; SCREEN_DATA_SIZE]);
    texture.set_filter(FilterMode::Nearest);

    loop {
        if NEW_FRAME.swap(false, Ordering::Relaxed) {
            let p = NEW_FRAME_PIXELS.lock().unwrap();
            // println!("{:?}", &p[..40]);
            analyze_screen_data(*p);
            texture.update_from_bytes(128, 128, &p[..]);
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
