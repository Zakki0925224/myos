use core::{fmt::{self, Write}, ptr::{write_volatile, read_volatile}};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::device::serial::{SerialPort, IO_PORT_COM1};

const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;
const VGA_MEM: u32 = 0xb8000;

const TAB_CHAR: char = ' ';
const TAB_INDENT_SIZE: usize = 4;

lazy_static!
{
    pub static ref VGA_SCREEN: Mutex<VgaScreen> = Mutex::new(VgaScreen::new(Color::White, Color::Black, IO_PORT_COM1));
}

#[derive(Debug)]
pub enum Color
{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15
}

pub struct VgaScreen
{
    default_color_code: u8,
    current_color_code: u8,
    cursor_x: usize,
    cursor_y: usize,
    serial_port: SerialPort
}

fn convert_curosr_pos_to_offset(x: usize, y: usize) -> usize
{
    return ((y - 1) * VGA_WIDTH + (x - 1)) * 2;
}

impl VgaScreen
{
    pub fn new(fore_color: Color, back_color: Color, com_port: u32) -> VgaScreen
    {
        let mut serial_port = SerialPort::new(com_port);
        serial_port.init();
        let default_color_code = (back_color as u8) << 4 | (fore_color as u8);

        let mut screen = VgaScreen
        {
            default_color_code,
            current_color_code: default_color_code,
            cursor_x: 1,
            cursor_y: 1,
            serial_port
        };
        screen.cls();

        return screen;
    }

    pub fn set_color(&mut self, fore_color: Color, back_color: Color)
    {
        self.current_color_code = (back_color as u8) << 4 | (fore_color as u8);
    }

    pub fn set_fore_color(&mut self, fore_color: Color)
    {
        let c = self.current_color_code;
        self.current_color_code = c & 0xf0 | fore_color as u8;
    }

    pub fn set_back_color(&mut self, back_color: Color)
    {
        let c = self.current_color_code;
        self.current_color_code = (back_color as u8) << 4 | c & 0xf;
    }

    pub fn reset_color(&mut self)
    {
        self.current_color_code = self.default_color_code;
    }

    pub fn write_char(&mut self, c: char)
    {
        let offset = convert_curosr_pos_to_offset(self.cursor_x, self.cursor_y) as isize;

        match c
        {
            '\n' => self.new_line(),
            '\t' => self.horizontal_tab(),
            _ =>
            {
                self.write_data(c as u8, offset);
                self.write_data(self.current_color_code, offset + 1);
                self.write_to_serial(c);
                self.inc_cursor();
            }
        }
    }

    pub fn write_string(&mut self, s: &str)
    {
        for c in s.chars()
        {
            self.write_char(c);
        }
    }

    pub fn write_color_block(&mut self, color: Color)
    {
        let c = self.current_color_code;
        self.current_color_code = (color as u8) << 4 | c & 0x15;
        self.write_char(' ');
        self.write_char(' ');
        self.current_color_code = c;
    }

    pub fn cls(&mut self)
    {
        for i in 0..VGA_HEIGHT * VGA_WIDTH * 2
        {
            self.write_data(0, i as isize);
        }

        self.cursor_x = 1;
        self.cursor_y = 1;
    }

    // TODO: support escape sequence
    fn write_to_serial(&self, c: char)
    {
        self.serial_port.send_data(c as u8);
    }

    fn write_data(&mut self, data: u8, offset: isize)
    {
        unsafe
        {
            let buffer = VGA_MEM as *mut u8;
            write_volatile(buffer.offset(offset), data);
        }
    }

    fn read_data(&mut self, offset: isize) -> u8
    {
        unsafe
        {
            let buffer = VGA_MEM as *mut u8;
            return read_volatile(buffer.offset(offset));
        }
    }

    fn new_line(&mut self)
    {
        self.write_to_serial('\n');
        for _i in self.cursor_x..=VGA_WIDTH
        {
            self.inc_cursor();
        }
    }

    fn horizontal_tab(&mut self)
    {
        for _i in 0..TAB_INDENT_SIZE
        {
            self.write_char(TAB_CHAR);
        }
    }

    fn inc_cursor(&mut self)
    {
        self.cursor_x += 1;

        if self.cursor_x > VGA_WIDTH
        {
            self.cursor_x = 1;
            self.cursor_y += 1;
        }

        if self.cursor_y > VGA_HEIGHT
        {
            self.scroll();
            self.cursor_x = 1;
            self.cursor_y = VGA_HEIGHT;
        }
    }

    fn scroll(&mut self)
    {
        for i in convert_curosr_pos_to_offset(1, 2)..=convert_curosr_pos_to_offset(VGA_WIDTH, VGA_HEIGHT) + 1
        {
            let buffer = self.read_data(i as isize);
            self.write_data(buffer, (i - VGA_WIDTH * 2) as isize);
        }

        for i in convert_curosr_pos_to_offset(1, VGA_HEIGHT)..=convert_curosr_pos_to_offset(VGA_WIDTH, VGA_HEIGHT) + 1
        {
            self.write_data(0, i as isize);
        }
    }
}

impl fmt::Write for VgaScreen
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        self.write_string(s);
        return Ok(());
    }
}

// print!, println! macro
#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    VGA_SCREEN.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) => ($crate::arch::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}