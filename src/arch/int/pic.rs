use crate::{asm, println};

const PIC0_ICW1: u32 = 0x0020;
const PIC0_OCW2: u32 = 0x0020;
const PIC0_IMR: u32 = 0x0021;
const PIC0_ICW2: u32 = 0x0021;
const PIC0_ICW3: u32 = 0x0021;
const PIC0_ICW4: u32 = 0x0021;
const PIC1_ICW1: u32 = 0x00a0;
const PIC1_OCW2: u32 = 0x00a0;
const PIC1_IMR: u32 = 0x00a1;
const PIC1_ICW2: u32 = 0x00a1;
const PIC1_ICW3: u32 = 0x00a1;
const PIC1_ICW4: u32 = 0x00a1;

pub fn init()
{
    asm::out8(PIC0_IMR, 0xff); // すべての割り込みを受け付けない
    asm::out8(PIC1_IMR, 0xff); // すべての割り込みを受け付けない

    asm::out8(PIC0_ICW1, 0x11); // エッジトリガモード
    asm::out8(PIC0_ICW2, 0x20); // IRQ0-7は、INT20-27で受ける
    asm::out8(PIC0_ICW3, 1 << 2); // PIC1はIRQ2にて接続
    asm::out8(PIC0_ICW4, 0x01); // ノンバッファモード

    asm::out8(PIC1_ICW1, 0x11); // エッジトリガモード
    asm::out8(PIC1_ICW2, 0x28); // IRQ8-15は、INT28-2fで受ける
    asm::out8(PIC1_ICW3, 2); // PIC1はIRQ2にて接続
    asm::out8(PIC1_ICW4, 0x01); // ノンバッファモード

    asm::out8(PIC0_IMR, 0xfb); // 11111011 PIC1以外はすべて禁止
    asm::out8(PIC1_IMR, 0xff); // 11111111 すべての割り込みを受け付けない
}

pub fn allow_input()
{
    asm::out8(PIC0_IMR, 0xf9); // allow PIC1 and keyboard
    asm::out8(PIC1_IMR, 0xef); // allow mouse
}

// PS/2 keyboard
pub extern "C" fn inthandler21()
{
    println!("INT 21");
    asm::test();

    loop { asm::hlt(); }
}

// mouse
pub extern "C" fn inthandler2c()
{
    println!("INT 2C");

    loop { asm::hlt(); }
}