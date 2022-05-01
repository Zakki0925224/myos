// intel 8259A interrupt controller

use crate::println;

use super::asm;

const MASTER_PIC_ADDR: u32 = 0x0020;
const SLAVE_PIC_ADDR: u32 = 0x00a0;

const PIC_IMR_MASK_IRQ0: u8 = 0x01;
const PIC_IMR_MASK_IRQ1: u8 = 0x02;
const PIC_IMR_MASK_IRQ2: u8 = 0x04;
const PIC_IMR_MASK_IRQ3: u8 = 0x08;
const PIC_IMR_MASK_IRQ4: u8 = 0x10;
const PIC_IMR_MASK_IRQ5: u8 = 0x20;
const PIC_IMR_MASK_IRQ6: u8 = 0x40;
const PIC_IMR_MASK_IRQ7: u8 = 0x80;
const PIC_IMR_MASK_IRQ_ALL: u8 = 0xff;

const EOI_COMMAND: u8 = 0x20;

pub const INT_VECTOR_IRQ0: i32 = 0x00000020;
pub const INT_VECTOR_IRQ1: i32 = 0x00000021;
pub const INT_VECTOR_IRQ2: i32 = 0x00000022;
pub const INT_VECTOR_IRQ3: i32 = 0x00000023;
pub const INT_VECTOR_IRQ4: i32 = 0x00000024;
pub const INT_VECTOR_IRQ5: i32 = 0x00000025;
pub const INT_VECTOR_IRQ6: i32 = 0x00000026;
pub const INT_VECTOR_IRQ7: i32 = 0x00000027;
pub const INT_VECTOR_IRQ8: i32 = 0x00000028;
pub const INT_VECTOR_IRQ9: i32 = 0x00000029;
pub const INT_VECTOR_IRQ10: i32 = 0x0000002a;
pub const INT_VECTOR_IRQ11: i32 = 0x0000002b;
pub const INT_VECTOR_IRQ12: i32 = 0x0000002c;
pub const INT_VECTOR_IRQ13: i32 = 0x0000002d;
pub const INT_VECTOR_IRQ14: i32 = 0x0000002e;
pub const INT_VECTOR_IRQ15: i32 = 0x0000002f;

pub fn init_pic()
{
    asm::out8(MASTER_PIC_ADDR, 0x11); // write ICW1 to master PIC
    asm::out8(SLAVE_PIC_ADDR, 0x11); // write ICW1 to slave PIC

    asm::out8(MASTER_PIC_ADDR + 1, 0x20); // write ICW2 to master PIC
    asm::out8(SLAVE_PIC_ADDR + 1, 0x28); // write ICW2 to slave PIC

    asm::out8(MASTER_PIC_ADDR + 1, 0x04); // write ICW3 to master PIC
    asm::out8(SLAVE_PIC_ADDR + 1, 0x02); // write ICW3 to slave PIC

    asm::out8(MASTER_PIC_ADDR + 1, 0x01); // write ICW4 to master PIC
    asm::out8(SLAVE_PIC_ADDR + 1, 0x01); // write ICW4 to slave PIC

    // mask all IRQs
    asm::out8(MASTER_PIC_ADDR + 1, !PIC_IMR_MASK_IRQ0 & !PIC_IMR_MASK_IRQ2);
    asm::out8(SLAVE_PIC_ADDR + 1, PIC_IMR_MASK_IRQ_ALL);

    // unmask IRQ1
    asm::out8(MASTER_PIC_ADDR + 1, !PIC_IMR_MASK_IRQ0 & !PIC_IMR_MASK_IRQ1 & !PIC_IMR_MASK_IRQ2);
    asm::out8(SLAVE_PIC_ADDR + 1, PIC_IMR_MASK_IRQ_ALL);
}

/// PS/2 keyboard interrupt
pub extern "C" fn keyboard_int()
{
    println!("keyboard_int");
    done_int();
}

fn done_int()
{
    // write EOI command to PIC
    asm::out8(MASTER_PIC_ADDR, EOI_COMMAND);
    asm::out8(SLAVE_PIC_ADDR, EOI_COMMAND);
}