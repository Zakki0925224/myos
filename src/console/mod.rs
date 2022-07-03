use crate::{print, println, util::logger::*, data::fifo::Fifo, device::{PCI, AHCI}, meta, mem, arch::vga::{VGA_SCREEN, Color}};
use lazy_static::lazy_static;
use spin::Mutex;

use self::ascii::AsciiCode;

pub mod ascii;

const CONSOLE_INPUT_CHARS_LIMIT: usize = 5;

lazy_static!
{
    static ref INPUTBUF: Mutex<Fifo> = Mutex::new(Fifo::new(128));
}

pub struct SystemConsole
{
    is_waiting_input: bool,
    input_cnt: u32
}

impl SystemConsole
{
    pub fn new() -> SystemConsole
    {
        return SystemConsole { is_waiting_input: false, input_cnt: 0 };
    }

    pub fn start(&mut self)
    {
        log_info("Starting built-in console...");
        VGA_SCREEN.lock().write_color_block(Color::Black);
        VGA_SCREEN.lock().write_color_block(Color::Blue);
        VGA_SCREEN.lock().write_color_block(Color::Green);
        VGA_SCREEN.lock().write_color_block(Color::Cyan);
        VGA_SCREEN.lock().write_color_block(Color::Red);
        VGA_SCREEN.lock().write_color_block(Color::Magenta);
        VGA_SCREEN.lock().write_color_block(Color::Brown);
        VGA_SCREEN.lock().write_color_block(Color::LightGray);
        VGA_SCREEN.lock().write_color_block(Color::DarkGray);
        VGA_SCREEN.lock().write_color_block(Color::LightBlue);
        VGA_SCREEN.lock().write_color_block(Color::LightGreen);
        VGA_SCREEN.lock().write_color_block(Color::LightCyan);
        VGA_SCREEN.lock().write_color_block(Color::LightRed);
        VGA_SCREEN.lock().write_color_block(Color::LightMagenta);
        VGA_SCREEN.lock().write_color_block(Color::Yellow);
        VGA_SCREEN.lock().write_color_block(Color::White);
        self.wait_input();
    }

    pub fn input_char(&mut self, ascii_code: AsciiCode)
    {
        if INPUTBUF.lock().free.get() == 0
        {
            print!("\n");
            log_warn("Reset input because input has exceeded buffer");
            self.wait_input();
            return;
        }

        if ascii_code == AsciiCode::NewLine
        {
            self.parse_input();
            self.wait_input();
            return;
        }

        INPUTBUF.lock().put(ascii_code as u8);
        self.input_cnt += 1;
        print!("{}", ascii_code as u8 as char);
    }

    pub fn is_waiting_input(&self) -> bool
    {
        return self.is_waiting_input;
    }

    fn wait_input(&mut self)
    {
        print!("\n");
        print!("# ");
        INPUTBUF.lock().clear();
        self.input_cnt = 0;
        self.is_waiting_input = true;
    }

    fn parse_input(&mut self)
    {
        if INPUTBUF.lock().status() == 0 || ((self.input_cnt as usize) < CONSOLE_INPUT_CHARS_LIMIT)
        {
            println!("\nUnknown command");
            return;
        }

        let mut chars = [0x0 as char; CONSOLE_INPUT_CHARS_LIMIT];

        for i in 0..CONSOLE_INPUT_CHARS_LIMIT
        {
            chars[i] = INPUTBUF.lock().get().unwrap() as char;
        }

        match chars
        {
            ['l', 's', 'p', 'c', 'i'] => self.do_process(|| PCI.lock().lspci()),
            ['i', 'a', 'h', 'c', 'i'] => self.do_process(|| AHCI.lock().ahci_info()),
            ['m', 'f', 'r', 'e', 'e'] => self.do_process(|| mem::free()),
            ['k', 'm', 'e', 't', 'a'] => self.do_process(|| meta::print_info()),
            ['c', 'l', 'e', 'a', 'r'] => self.do_process(|| VGA_SCREEN.lock().cls()),
            _ => println!("\nUnknown command")
        }
    }

    fn do_process<F: Fn()>(&mut self, func: F)
    {
        self.is_waiting_input = false;
        print!("\n");
        //log_info("Processing...");
        func();
        //log_info("Done");
    }
}