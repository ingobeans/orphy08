use crate::VIDEO_DATA_SIZE;

static FONT_DATA: &[u8] = include_bytes!("../../../font/font.bmp");
const FONT_CHAR_WIDTH: u8 = 5;
const FONT_CHAR_HEIGHT: u8 = 7;
const FONT_TOTAL_WIDTH: usize = 495;
static DOUBLE_WIDTH_CHARS: [u8; 4] = [b'M', b'W', b'm', b'w'];
static HALF_WIDTH_CHARS: [u8; 5] = [b'i', b'l', b'\'', b',', b'.'];

/// Returns u8::MAX if not found, otherwise the index of the character in the fontsheet.
/// Multiply by 5 to get the actual X coordinate
fn get_pos_of_char(mut char: u8) -> u8 {
    if char < 33 || char > 126 {
        return u8::MAX;
    }
    let mut add = 0;
    for c in DOUBLE_WIDTH_CHARS {
        if char > c {
            add += 1;
        }
    }
    char += add;
    (char - 33) as _
}

#[inline]
fn get_font_data_pixel(x: usize, y: usize) -> u8 {
    const HEADER_SIZE: usize = 62;
    // because lines are rounded to 4 bytes
    let stride = (FONT_TOTAL_WIDTH + 31) / 32 * 4;
    // y is flipped
    let y = 7 - 1 - y;
    FONT_DATA[HEADER_SIZE + y * stride + x / 8] & (0b10000000 >> (x % 8))
}

pub fn write_error(str: &str, video_memory: *mut u8) {
    unsafe {
        let slice: &mut [[u8; 4]] =
            std::slice::from_raw_parts_mut(video_memory as _, VIDEO_DATA_SIZE / 4);
        slice.fill([0, 0, 0, 0]);
        write_str_to_buffer(
            str,
            slice,
            128,
            2,
            2,
            [255, 255, 255, 255],
            Some([0, 0, 0, 255]),
        );
    }
}

pub fn write_str_to_buffer<T: Copy>(
    str: &str,
    buf: &mut [T],
    buf_width: usize,
    mut draw_x: usize,
    mut draw_y: usize,
    on_value: T,
    off_value: Option<T>,
) {
    let start_x = draw_x;
    let mut chars = str.chars().peekable();
    while let Some(char) = chars.next() {
        if char == '\n' {
            draw_y += FONT_CHAR_HEIGHT as usize + 2;
            draw_x = start_x;
            continue;
        }
        draw_x += write_char_to_buffer(
            char as u8, buf, buf_width, draw_x, draw_y, on_value, off_value,
        ) as usize;

        if let Some(c) = chars.peek()
            && draw_x >= 128 - get_width_of(*c as u8) as usize
        {
            draw_y += FONT_CHAR_HEIGHT as usize + 2;
            draw_x = start_x;
            continue;
        }
    }
}

#[inline]
fn get_width_of(char: u8) -> u8 {
    if DOUBLE_WIDTH_CHARS.contains(&char) {
        FONT_CHAR_WIDTH * 2
    } else if HALF_WIDTH_CHARS.contains(&char) {
        FONT_CHAR_WIDTH / 2
    } else {
        FONT_CHAR_WIDTH
    }
}

pub fn write_char_to_buffer<T: Copy>(
    char: u8,
    buf: &mut [T],
    buf_width: usize,
    draw_x: usize,
    draw_y: usize,
    on_value: T,
    off_value: Option<T>,
) -> u8 {
    if char == b' ' {
        return FONT_CHAR_WIDTH;
    }
    let mut pos = get_pos_of_char(char);
    if pos == u8::MAX {
        pos = 127 - 33;
    }

    let w = get_width_of(char);

    for x in 0..w {
        for y in 0..FONT_CHAR_HEIGHT {
            let pixel = get_font_data_pixel(
                x as usize + pos as usize * FONT_CHAR_WIDTH as usize,
                y as usize,
            );
            let pixel = if pixel != 0 {
                on_value
            } else {
                if let Some(v) = off_value {
                    v
                } else {
                    continue;
                }
            };
            buf[draw_x + x as usize + (draw_y + y as usize) * buf_width] = pixel;
        }
    }

    w
}
