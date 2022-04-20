use core::fmt;

const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;
const VGA_MEM: u32 = 0xb8000;

const TAB_CHAR: char = ' ';
const TAB_INDENT_SIZE: usize = 4;

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
    color_code: u8,
    cursor_x: usize,
    cursor_y: usize
}

fn convert_curosr_pos_to_offset(x: usize, y: usize) -> usize
{
    return ((y - 1) * VGA_WIDTH + (x - 1)) * 2;
}

impl VgaScreen
{
    pub fn new(fore_color: Color, back_color: Color) -> VgaScreen
    {
        return VgaScreen
        {
            color_code: (back_color as u8) << 4 | (fore_color as u8),
            cursor_x: 1,
            cursor_y: 1
        };
    }

    pub fn set_color(&mut self, fore_color: Color, back_color: Color)
    {
        self.color_code = (back_color as u8) << 4 | (fore_color as u8);
    }

    pub fn write_char(&mut self, c: char)
    {
        let buffer = VGA_MEM as *mut u8;
        let offset = convert_curosr_pos_to_offset(self.cursor_x, self.cursor_y) as isize;

        match c
        {
            '\n' => self.new_line(),
            '\t' => self.horizontal_tab(),
            _ =>
            {
                self.write_data(c as u8, offset);
                self.write_data(self.color_code, offset + 1);
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

    pub fn cls(&mut self)
    {
        let buffer = VGA_MEM as *mut u8;

        for i in 0..VGA_HEIGHT * VGA_WIDTH * 2
        {
            self.write_data(0, i as isize);
        }

        self.cursor_x = 1;
        self.cursor_y = 1;
    }

    fn write_data(&mut self, data: u8, offset: isize)
    {
        unsafe
        {
            let buffer = VGA_MEM as *mut u8;
            unsafe { *buffer.offset(offset) = data; }
        }
    }

    fn read_data(&mut self, offset: isize) -> u8
    {
        unsafe
        {
            let buffer = VGA_MEM as *mut u8;
            return unsafe { *buffer.offset(offset) };
        }
    }

    fn new_line(&mut self)
    {
        for i in self.cursor_x..=VGA_WIDTH
        {
            self.inc_cursor();
        }
    }

    fn horizontal_tab(&mut self)
    {
        for i in 0..TAB_INDENT_SIZE
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