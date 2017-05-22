use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($args:tt)*) => (print!(concat!($fmt, "\n"), $($args)*));
}

macro_rules! print {
    ($($args:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($args)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear_screen() {
    WRITER.lock().clear_screen();
}

static WRITER: Mutex<Writer> = Mutex::new(Writer {
    curr_col: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe {Unique::new(0xb8000 as *mut _ )}
});

struct Writer {
    curr_col: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

// Buffer is defined for easy indexing to VGA Buffer
type Buffer = [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

// Defines a single character in the VGA Buffer
#[derive(Copy, Clone)]
#[repr(C)]
struct VGAChar {
    character: u8,
    color_code: ColorCode,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Copy, Clone)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.write_byte(c);
        }
        Ok(())
    }
}

impl Writer {
    // Writes a byte to the console with self's terminal color.
    // If the end of line is reached text wraps around.
    pub fn write_byte(&mut self, ch: u8) {
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
                self.buffer()[row][col].write(VGAChar { 
                    character: ch, 
                    color_code: color_code
                });
                self.curr_col += 1;
            }
        }
    }

    // Each line is shifted up by one. The topmost line will be erased.
    // The cursor will be placed in the bottom right corner
    pub fn new_line(&mut self) {
        self.curr_col = 0;

        //0th row gets erased
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer()[row][col].read();
                self.buffer()[row-1][col].write(ch);
                let color = self.color_code;
                self.buffer()[row][col].write(VGAChar{ 
                    character: b' ',
                    color_code: color
                });
            }
        }
    }

    pub fn clear_screen(&mut self) {
        for _ in 0..BUFFER_HEIGHT {
            self.new_line()
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }
}
