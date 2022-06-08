use crate::{arch::vga::{VGA_SCREEN, Color}, println};

pub fn log_debug<T: core::fmt::Debug>(msg: &str, info: T)
{
    set_vga_color(Color::Cyan);
    println!("[DBG]: {}: {:?}", msg, info);
    reset_vga_color();
}

pub fn log_info(msg: &str)
{
    set_vga_color(Color::Yellow);
    println!("[INF]: {}", msg);
    reset_vga_color();
}

pub fn log_warn(msg: &str)
{
    set_vga_color(Color::Magenta);
    println!("[WRN]: {}", msg);
    reset_vga_color();
}

pub fn log_error(msg: &str)
{
    set_vga_color(Color::Red);
    println!("[ERR]: {}", msg);
    reset_vga_color();
}

fn set_vga_color(fore_color: Color)
{
    VGA_SCREEN.lock().set_color(fore_color, Color::Black);
}

fn reset_vga_color()
{
    VGA_SCREEN.lock().set_color(Color::White, Color::Black);
}