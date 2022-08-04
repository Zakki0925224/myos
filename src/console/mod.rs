use crate::{print, println, util::logger::*, data::fifo::Fifo, device::{PCI, AHCI}, meta, mem, arch::{vga::{VGA_SCREEN, Color}, asm}};
use alloc::{vec::Vec, string::{String, ToString}};
use lazy_static::lazy_static;
use spin::Mutex;

use self::ascii::AsciiCode;

pub mod ascii;

const CONSOLE_INPUT_CHARS_LIMIT: usize = 128;

pub struct SystemConsole
{
    is_waiting_input: bool,
    input_buf: Vec<char>
}

impl SystemConsole
{
    pub fn new() -> SystemConsole
    {
        return SystemConsole
        {
            is_waiting_input: false,
            input_buf: Vec::new()
        }
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
        if self.input_buf.len() > CONSOLE_INPUT_CHARS_LIMIT
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

        self.input_buf.push(ascii_code as u8 as char);
        print!("{}", ascii_code as u8 as char);
    }

    pub fn is_waiting_input(&self) -> bool
    {
        return self.is_waiting_input;
    }

    fn wait_input(&mut self)
    {
        self.is_waiting_input = true;
        print!("\n");
        print!("# ");

        if self.input_buf.len() != 0
        {
            self.input_buf.clear();
        }
    }

    fn parse_input(&mut self)
    {
        if self.input_buf.len() == 0
        {
            return;
        }

        let input = self.input_buf.iter().collect::<String>();

        // TODO: make command list
        match input.as_str()
        {
            "lspci" => self.do_process(|| PCI.lock().lspci()),
            "iahci" => self.do_process(|| AHCI.lock().ahci_info()),
            "mfree" => self.do_process(|| mem::free()),
            "minfo" => self.do_process(|| mem::info()),
            "kmeta" => self.do_process(|| meta::print_info()),
            "clear" => self.do_process(|| VGA_SCREEN.lock().cls()),
            "itest" => self.do_process(|| asm::test()),
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