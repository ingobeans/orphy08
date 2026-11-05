static FONT_DATA: &[u8] = include_bytes!("../../../font/font.bmp");
const FONT_CHAR_WIDTH: u8 = 5;
const FONT_CHAR_HEIGHT: u8 = 7;
const FONT_TOTAL_WIDTH: usize = 495;
static DOUBLE_WIDTH_CHARS: [u8; 4] = [b'M', b'W', b'm', b'w'];

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

pub fn write_str_to_buffer<T: Copy>(
    str: &str,
    buf: &mut [T],
    buf_width: usize,
    mut draw_x: usize,
    mut draw_y: usize,
    on_value: T,
    off_value: Option<T>,
) {
    for char in str.chars() {
        if char == '\n' {
            draw_y += FONT_CHAR_HEIGHT as usize;
            continue;
        }
        draw_x += write_char_to_buffer(
            char as u8, buf, buf_width, draw_x, draw_y, on_value, off_value,
        ) as usize;
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

    let w = if DOUBLE_WIDTH_CHARS.contains(&char) {
        FONT_CHAR_WIDTH * 2
    } else {
        FONT_CHAR_WIDTH
    };

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
