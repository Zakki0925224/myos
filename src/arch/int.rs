// intel 8259A interrupt controller on PC/AT

use crate::{println, arch::vga::{VGA_SCREEN, Color}};

use super::asm;

const MASTER_PIC_ADDR: u32 = 0x0020;
const SLAVE_PIC_ADDR: u32 = 0x00a0;
const DISALLOW_ALL_INTERRUPTS: u8 = 0xff;
const EDGE_TRIGGER_MODE: u8 = 0x11;
const NONE_BUFFER_MODE: u8 = 0x01;
const EOI_COMMAND: u8 = 0x20;

// master pic
pub const INT_VECTOR_IRQ0: i32 = 0x20;    // system timer
pub const INT_VECTOR_IRQ1: i32 = 0x21;    // PS/2 keyboard
pub const INT_VECTOR_IRQ2: i32 = 0x22;    // cascade
pub const INT_VECTOR_IRQ3: i32 = 0x23;    // serial port COM2 and COM4
pub const INT_VECTOR_IRQ4: i32 = 0x24;    // serial port COM1 and COM3
pub const INT_VECTOR_IRQ5: i32 = 0x25;    // LPT2
pub const INT_VECTOR_IRQ6: i32 = 0x26;    // floppy disk controller
pub const INT_VECTOR_IRQ7: i32 = 0x27;    // LPT1

// slave pic
pub const INT_VECTOR_IRQ8: i32 = 0x28;    // real-time clock
pub const INT_VECTOR_IRQ9: i32 = 0x29;    // free
pub const INT_VECTOR_IRQ10: i32 = 0x2a;   // free
pub const INT_VECTOR_IRQ11: i32 = 0x2b;   // free
pub const INT_VECTOR_IRQ12: i32 = 0x2c;   // PS/2 mouse
pub const INT_VECTOR_IRQ13: i32 = 0x2d;   // coprocessor
pub const INT_VECTOR_IRQ14: i32 = 0x2e;   // HDD conteroller
pub const INT_VECTOR_IRQ15: i32 = 0x2f;   // HDD controller

pub fn init_pic()
{
    asm::out8(MASTER_PIC_ADDR + 1, DISALLOW_ALL_INTERRUPTS);
    asm::out8(SLAVE_PIC_ADDR + 1, DISALLOW_ALL_INTERRUPTS);

    // IRQ0-7 are mapped to IDT entries 0x20-0x27
    asm::out8(MASTER_PIC_ADDR, EDGE_TRIGGER_MODE);
    asm::out8(MASTER_PIC_ADDR + 1, 0x20);
    asm::out8(MASTER_PIC_ADDR + 1, 1 << 2);
    asm::out8(MASTER_PIC_ADDR + 1, NONE_BUFFER_MODE);

    // IRQ8-15 are mapped to IDT entries 0x28-0x2f
    asm::out8(SLAVE_PIC_ADDR, EDGE_TRIGGER_MODE);
    asm::out8(SLAVE_PIC_ADDR + 1, 0x28);
    asm::out8(SLAVE_PIC_ADDR + 1, 2);
    asm::out8(SLAVE_PIC_ADDR + 1, NONE_BUFFER_MODE);

    // mask all
    asm::out8(MASTER_PIC_ADDR + 1, 0xfb);
    asm::out8(SLAVE_PIC_ADDR + 1, 0xff);

    // allow interrupts
    asm::out8(MASTER_PIC_ADDR + 1, 0xf9);   // allow IRQ0-7
    asm::out8(SLAVE_PIC_ADDR + 1, 0xef);    // allow IRQ8-15

    println!("PIC initialized");
}

/// PS/2 keyboard interrupt
pub extern "C" fn keyboard_int()
{
    notice_reception_complate(INT_VECTOR_IRQ1);
    let data = asm::in8(0x60);
    VGA_SCREEN.lock().set_color(Color::Yellow, Color::Black);
    println!("[INT]: IRQ-1 (PS/2 keyboard), data: 0x{:x}", data);
    VGA_SCREEN.lock().set_color(Color::White, Color::Black);
    //done_int();

    //loop { asm::hlt(); }
}

/// PS/2 mouse interrupt
pub extern "C" fn mouse_int()
{
    notice_reception_complate(INT_VECTOR_IRQ12);
    VGA_SCREEN.lock().set_color(Color::Yellow, Color::Black);
    println!("[INT]: IRQ-12 (PS/2 mouse)");
    VGA_SCREEN.lock().set_color(Color::White, Color::Black);
    //done_int();

    //loop { asm::hlt(); }
}

/// resume next interrupt
fn notice_reception_complate(port: i32)
{
    if port > INT_VECTOR_IRQ7
    {
        // master pic
        asm::out8(MASTER_PIC_ADDR, 0x60 + (port - INT_VECTOR_IRQ0) as u8);
    }
    else
    {
        // slave pic
        asm::out8(SLAVE_PIC_ADDR, 0x60 + (port - INT_VECTOR_IRQ0) as u8);
    }
}

fn done_int()
{
    // write EOI command to PIC
    asm::out8(MASTER_PIC_ADDR, EOI_COMMAND);
    asm::out8(SLAVE_PIC_ADDR, EOI_COMMAND);
}