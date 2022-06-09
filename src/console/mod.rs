use crate::{print, println, util::logger::{log_info, log_debug, log_warn}, data::fifo::Fifo};
use lazy_static::lazy_static;
use spin::Mutex;

use self::ascii::AsciiCode;

pub mod ascii;

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
            self.do_process();
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
        print!("root$ ");
        INPUTBUF.lock().clear();
        self.input_cnt = 0;
        self.is_waiting_input = true;
    }

    fn do_process(&mut self)
    {
        self.is_waiting_input = false;
        print!("\n");
        log_info("Processing...");

        for i in 0..self.input_cnt
        {
            let c = INPUTBUF.lock().get().unwrap();
            print!("{}", c as char);
        }
    }
}