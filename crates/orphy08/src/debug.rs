static FONT_DATA: &[u8] = include_bytes!("../../../font/font.bmp");
const FONT_CHAR_WIDTH: u8 = 5;
const FONT_CHAR_HEIGHT: u8 = 7;
const FONT_TOTAL_WIDTH: usize = 495;

/// Returns u8::MAX if not found, otherwise the index of the character in the fontsheet.
/// Multiply by 5 to get the actual X coordinate
fn get_pos_of_char(char: u8) -> u8 {
    if char < 33 || char > 126 {
        return u8::MAX;
    }
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

    for x in 0..FONT_CHAR_WIDTH {
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

    0
}
