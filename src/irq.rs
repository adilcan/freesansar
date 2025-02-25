#![no_std]

use core::ptr;

type IRQHandler = fn(i32);

#[derive(Copy, Clone)]
pub struct SigAction {
    pub sa_handler: Option<IRQHandler>,
    pub sa_flags: u32,
    pub sa_mask: u32,
}

static mut IRQ_SIGACTION: [SigAction; 16] = [SigAction { sa_handler: None, sa_flags: 0, sa_mask: 0 }; 16];

static mut CACHE_21: u8 = 0xff;
static mut CACHE_A1: u8 = 0xff;
static mut INTR_COUNT: u64 = 0;
static mut BH_ACTIVE: u32 = 0;
const BH_MASK: u32 = 0xFFFFFFFF;

pub struct BHStruct {
    pub routine: Option<fn(data: *mut ())>,
    pub data: *mut (),
}

static mut BH_BASE: [BHStruct; 32] = [BHStruct { routine: None, data: ptr::null_mut() }; 32];

unsafe fn cli() {
    core::arch::asm!("cli");
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn disable_irq(irq_nr: u32) {
    let mask = 1 << (irq_nr & 7);
    cli();
    if irq_nr < 8 {
        CACHE_21 |= mask as u8;
        outb(0x21, CACHE_21);
    } else {
        CACHE_A1 |= mask as u8;
        outb(0xA1, CACHE_A1);
    }
}

pub unsafe fn enable_irq(irq_nr: u32) {
    let mask = !(1 << (irq_nr & 7));
    cli();
    if irq_nr < 8 {
        CACHE_21 &= mask as u8;
        outb(0x21, CACHE_21);
    } else {
        CACHE_A1 &= mask as u8;
        outb(0xA1, CACHE_A1);
    }
}

pub unsafe fn do_bottom_half() {
    let active = BH_ACTIVE & BH_MASK;
    for i in 0..32 {
        let mask = 1 << i;
        if (active & mask) != 0 {
            BH_ACTIVE &= !mask;
            if let Some(routine) = BH_BASE[i].routine {
                routine(BH_BASE[i].data);
            } else {
                panic!("irq.rs: bad_bottom_half_entry");
            }
        }
    }
}

pub unsafe fn do_IRQ(irq: u32, regs: usize) {
    if (irq as usize) < IRQ_SIGACTION.len() {
        if let Some(handler) = IRQ_SIGACTION[irq as usize].sa_handler {
            INTR_COUNT += 1;
            handler(regs as i32);
        }
    }
}

pub unsafe fn do_fast_IRQ(irq: u32) {
    if (irq as usize) < IRQ_SIGACTION.len() {
        if let Some(handler) = IRQ_SIGACTION[irq as usize].sa_handler {
            INTR_COUNT += 1;
            handler(irq as i32);
        }
    }
}

pub unsafe fn irqaction(irq: u32, new_sa: SigAction) -> i32 {
    if irq > 15 {
        return -1;
    }
    let sa = &mut IRQ_SIGACTION[irq as usize];
    if sa.sa_mask != 0 {
        return -2;
    }
    if new_sa.sa_handler.is_none() {
        return -1;
    }
    *sa = new_sa;
    sa.sa_mask = 1;
    if irq < 8 {
        CACHE_21 &= !(1 << irq) as u8;
        outb(0x21, CACHE_21);
    } else {
        CACHE_21 &= !(1 << 2) as u8;
        CACHE_A1 &= !(1 << (irq - 8)) as u8;
        outb(0x21, CACHE_21);
        outb(0xA1, CACHE_A1);
    }
    0
}

pub unsafe fn request_irq(irq: u32, handler: IRQHandler) -> i32 {
    let new_sa = SigAction {
        sa_handler: Some(handler),
        sa_flags: 0,
        sa_mask: 0,
    };
    irqaction(irq, new_sa)
}

pub unsafe fn free_irq(irq: u32) {
    if irq > 15 {
        return;
    }
    let sa = &mut IRQ_SIGACTION[irq as usize];
    if sa.sa_mask == 0 {
        return;
    }
    cli();
    if irq < 8 {
        CACHE_21 |= 1 << irq;
        outb(0x21, CACHE_21);
    } else {
        CACHE_A1 |= 1 << (irq - 8);
        outb(0xA1, CACHE_A1);
    }
    sa.sa_handler = None;
    sa.sa_flags = 0;
    sa.sa_mask = 0;
}

fn no_action(_irq: i32) {}

fn math_error_irq(cpl: i32) {
    unsafe {
        outb(0xF0, 0);
    }
    panic!("math error irqq triggy: CPL {}", cpl);
}

pub unsafe fn init_IRQ() {
    for i in 0..16 {
    }
    let ignore_irq = SigAction {
        sa_handler: Some(no_action),
        sa_flags: 1, // 0x1
        sa_mask: 0,
    };
    if irqaction(2, ignore_irq) != 0 {
        // todotodo
    }
    if request_irq(13, math_error_irq) != 0 {
        // todotodo
    }
    for i in 0..32 {
        BH_BASE[i].routine = None;
        BH_BASE[i].data = ptr::null_mut();
    }
    BH_ACTIVE = 0;
    INTR_COUNT = 0;
}
