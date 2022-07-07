// intel 8259A interrupt controller on PC/AT

use crate::{data::fifo::Fifo, util::logger::*};
use lazy_static::lazy_static;
use spin::Mutex;

use super::asm;

lazy_static!
{
    pub static ref KEYBUF: Mutex<Fifo> = Mutex::new(Fifo::new(32));
}

lazy_static!
{
    pub static ref MOUSEBUF: Mutex<Fifo> = Mutex::new(Fifo::new(32));
}

const MASTER_PIC_ADDR: u32 = 0x0020;
const SLAVE_PIC_ADDR: u32 = 0x00a0;
const DISALLOW_ALL_INTERRUPTS: u8 = 0xff;
const EDGE_TRIGGER_MODE: u8 = 0x11;
const NONE_BUFFER_MODE: u8 = 0x01;
const EOI_COMMAND: u8 = 0x20;

// master pic
pub const INT_VECTOR_IRQ0: u32 = 0x20;    // system timer
pub const INT_VECTOR_IRQ1: u32 = 0x21;    // PS/2 keyboard
pub const INT_VECTOR_IRQ2: u32 = 0x22;    // cascade
pub const INT_VECTOR_IRQ3: u32 = 0x23;    // serial port COM2 and COM4
pub const INT_VECTOR_IRQ4: u32 = 0x24;    // serial port COM1 and COM3
pub const INT_VECTOR_IRQ5: u32 = 0x25;    // LPT2
pub const INT_VECTOR_IRQ6: u32 = 0x26;    // floppy disk controller
pub const INT_VECTOR_IRQ7: u32 = 0x27;    // LPT1

// slave pic
pub const INT_VECTOR_IRQ8: u32 = 0x28;    // real-time clock
pub const INT_VECTOR_IRQ9: u32 = 0x29;    // free
pub const INT_VECTOR_IRQ10: u32 = 0x2a;   // free
pub const INT_VECTOR_IRQ11: u32 = 0x2b;   // free
pub const INT_VECTOR_IRQ12: u32 = 0x2c;   // PS/2 mouse
pub const INT_VECTOR_IRQ13: u32 = 0x2d;   // coprocessor
pub const INT_VECTOR_IRQ14: u32 = 0x2e;   // HDD conteroller
pub const INT_VECTOR_IRQ15: u32 = 0x2f;   // HDD controller

// mouse
const PORT_KEYDAT: u32 = 0x0060;
const PORT_KEYCMD: u32 = 0x0064;
const PORT_KEYSTA: u32 = 0x0064;
const KEYSTA_SEND_NOT_READY: u8 = 0x02;
const KEYCMD_WRITE_MODE: u8 = 0x60;
const KBC_MODE: u8 = 0x47;
const KEYCMD_SENDTO_MOUSE: u8 = 0xd4;
const MOUSECMD_ENABLE: u8 = 0xf4;

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

    init_keyboard();

    log_info("PIC initialized");
}

fn init_keyboard()
{
    wait_kbc_send_ready();
    asm::out8(PORT_KEYCMD, KEYCMD_WRITE_MODE);
    wait_kbc_send_ready();
    asm::out8(PORT_KEYDAT, KBC_MODE);
}

pub fn enable_mouse()
{
    wait_kbc_send_ready();
    asm::out8(PORT_KEYCMD, KEYCMD_SENDTO_MOUSE);
    wait_kbc_send_ready();
    asm::out8(PORT_KEYDAT, MOUSECMD_ENABLE);
}

/// PS/2 keyboard interrupt
pub extern "C" fn keyboard_int()
{
    let data = asm::in8(0x60);
    KEYBUF.lock().put(data).unwrap();
    done_int();
}

/// PS/2 mouse interrupt
pub extern "C" fn mouse_int()
{
    let data = asm::in8(PORT_KEYDAT);
    MOUSEBUF.lock().put(data).unwrap();
    done_int();
}

fn done_int()
{
    // write EOI command to PIC
    asm::out8(MASTER_PIC_ADDR, EOI_COMMAND);
    asm::out8(SLAVE_PIC_ADDR, EOI_COMMAND);
}

fn wait_kbc_send_ready()
{
    // wait for data ready
    loop
    {
        if (asm::in8(PORT_KEYSTA) & KEYSTA_SEND_NOT_READY) == 0
        {
            break;
        }
    }
}