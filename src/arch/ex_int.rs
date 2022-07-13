use core::panic;

use crate::util::logger::log_warn;

use super::asm;

pub const EX_INT_DIVIDED_BY_ZERO: u32 = 0x0;
pub const EX_INT_SINGLE_STEP: u32 = 0x1;
pub const EX_INT_NMI: u32 = 0x2;
pub const EX_INT_BREAKPOINT: u32 = 0x3;
pub const EX_INT_OVERFLOW: u32 = 0x4;
pub const EX_INT_BOUND_RANGE_EXCEEDED: u32 = 0x5;
pub const EX_INT_INVALID_OPCODE: u32 = 0x6;
pub const EX_INT_CPROC_NOT_AVAILABLE: u32 = 0x7;
pub const EX_INT_DOUBLE_FAULT: u32 = 0x8;
pub const EX_INT_CPROC_SGM_OVERRUN: u32 = 0x9; // 386 or earlier only
pub const EX_INT_INVALID_TSS: u32 = 0xa;
pub const EX_INT_SGM_NOT_PRESENT: u32 = 0xb;
pub const EX_INT_STACK_SGM_FAULT: u32 = 0xc;
pub const EX_INT_GENERAL_PROTECTION_FAULT: u32 = 0xd;
pub const EX_INT_PAGE_FAULT: u32 = 0xe;
pub const EX_INT_FLOATING_POINT: u32 = 0x10;
pub const EX_INT_ALIGN_CHECK: u32 = 0x11;
pub const EX_INT_MACHINE_CHECK: u32 = 0x12;
pub const EX_INT_SIMD_FLOATING_POINT: u32 = 0x13;

/// divided by zero exception
pub extern "C" fn ex_divided_by_zero()
{
    panic!("Throw divided by zero exception.");
}

/// double fault
pub extern "C" fn ex_double_fault()
{
    panic!("Throw double fault exception.");
}

/// general protection fault
pub extern "C" fn ex_general_protection_fault()
{
    panic!("Throw general protection fault exception.");
}

/// page fault
pub extern "C" fn ex_page_fault()
{
    panic!("Throw page fault exception.");
}

/// breakpoint
pub extern "C" fn ex_breakpoint()
{
    panic!("Throw breakpoint exception.");
}