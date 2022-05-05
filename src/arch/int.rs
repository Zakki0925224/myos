// intel 8259A interrupt controller on PC/AT

use crate::println;

use super::asm;

const MASTER_PIC_ADDR: u32 = 0x0020;
const SLAVE_PIC_ADDR: u32 = 0x00a0;
const DISALLOW_ALL_INTERRUPTS: u8 = 0xff;
const EDGE_TRIGGER_MODE: u8 = 0x11;
const NONE_BUFFER_MODE: u8 = 0x01;
const EOI_COMMAND: u8 = 0x20;

// master pic
pub const INT_VECTOR_IRQ0: i32 = 0x00000020;    // system timer
pub const INT_VECTOR_IRQ1: i32 = 0x00000021;    // PS/2 keyboard
pub const INT_VECTOR_IRQ2: i32 = 0x00000022;    // cascade
pub const INT_VECTOR_IRQ3: i32 = 0x00000023;    // serial port COM2 and COM4
pub const INT_VECTOR_IRQ4: i32 = 0x00000024;    // serial port COM1 and COM3
pub const INT_VECTOR_IRQ5: i32 = 0x00000025;    // LPT2
pub const INT_VECTOR_IRQ6: i32 = 0x00000026;    // floppy disk controller
pub const INT_VECTOR_IRQ7: i32 = 0x00000027;    // LPT1

// slave pic
pub const INT_VECTOR_IRQ8: i32 = 0x00000028;    // real-time clock
pub const INT_VECTOR_IRQ9: i32 = 0x00000029;    // free
pub const INT_VECTOR_IRQ10: i32 = 0x0000002a;   // free
pub const INT_VECTOR_IRQ11: i32 = 0x0000002b;   // free
pub const INT_VECTOR_IRQ12: i32 = 0x0000002c;   // PS/2 mouse
pub const INT_VECTOR_IRQ13: i32 = 0x0000002d;   // coprocessor
pub const INT_VECTOR_IRQ14: i32 = 0x0000002e;   // HDD conteroller
pub const INT_VECTOR_IRQ15: i32 = 0x0000002f;   // HDD controller

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
    asm::out8(MASTER_PIC_ADDR + 1, 0xf9);
    asm::out8(SLAVE_PIC_ADDR + 1, 0xef);

    println!("PIC initialized");
}

/// PS/2 keyboard interrupt
pub extern "C" fn keyboard_int()
{
    println!("IRQ-1 (PS/2 keyboard)");

    loop { asm::hlt(); }
}

fn done_int()
{
    // write EOI command to PIC
    asm::out8(MASTER_PIC_ADDR, EOI_COMMAND);
    asm::out8(SLAVE_PIC_ADDR, EOI_COMMAND);
}