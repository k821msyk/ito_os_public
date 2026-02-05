//! VGA Text Mode Driver / VGAテキストモード・ドライバ

pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub fn write_vga(buffer: &[u8; 80 * 25 * 2]) {
    unsafe {
        core::ptr::copy_nonoverlapping(buffer.as_ptr(), VGA_BUFFER, 80 * 25 * 2);
    }
}

pub fn clear_buffer(buffer: &mut [u8; 80 * 25 * 2], color: u8) {
    for i in 0..80 * 25 {
        buffer[i * 2] = b' ';
        buffer[i * 2 + 1] = color;
    }
}

pub fn print_buffer(buffer: &mut [u8; 80 * 25 * 2], msg: &[u8], row: usize, col: usize, color: u8) {
    for (i, &b) in msg.iter().enumerate() {
        let idx = (row * 80 + col + i) * 2;
        if idx < 80 * 25 * 2 {
            buffer[idx] = b;
            buffer[idx + 1] = color;
        }
    }
}

pub fn draw_num(buffer: &mut [u8; 80 * 25 * 2], val: u64, row: usize, col: usize, color: u8) {
    let b = b'0' + (val % 10) as u8;
    let idx = (row * 80 + col) * 2;
    if idx < 80 * 25 * 2 {
        buffer[idx] = b;
        buffer[idx + 1] = color;
    }
}

pub fn draw_bar(buffer: &mut [u8; 80 * 25 * 2], row: usize, col: usize, val: u8, max: u8, c_full: u8, c_empty: u8) {
    for i in 0..max as usize {
        let color = if i < val as usize { c_full } else { c_empty };
        print_buffer(buffer, b" ", row, col + i, color);
    }
}