use core::ptr::Unique;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    curr_col: 0,
    color_code: ColorCode(0x0A),
    buffer: unsafe {Unique::new(0xb8000 as *mut _ )}
});

#[derive(Copy, Clone)]
struct ColorCode(u8);

type Buffer = [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT];

#[repr(C)]
struct VGAChar {
    character: u8,
    color_code: ColorCode,
}

pub struct Writer {
    curr_col: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {

    pub fn put_byte(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                self.new_line();
            }
            ch => {
                if self.curr_col >= BUFFER_WIDTH {
                    self.new_line();
                    self.curr_col = 0;
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.curr_col;
                let color_code = self.color_code;
                self.buffer()[row][col] = VGAChar{ character: ch, color_code: color_code};
            }
        }
    }

    fn new_line(&mut self) {
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }
}
