#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;
mod quantum;
mod kernel;

use core::panic::PanicInfo;
use quantum::{QuantumEnv, VirtualQuantumHAL};
use kernel::{QuantumResourceManager, QuantumError};

static mut BACK_BUFFER: [u8; 80 * 25 * 2] = [0; 80 * 25 * 2];
static mut Q_MGR: QuantumResourceManager = QuantumResourceManager::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut env = VirtualQuantumHAL::new();
    unsafe { vga::clear_buffer(&mut BACK_BUFFER, 0x00); }

    loop {
        unsafe {
            env.step();
            Q_MGR.decay_all(env.get_entropy());
            if let Err(e) = Q_MGR.check_stability(env.get_entropy()) {
                handle_panic(e);
            }
            render(&mut env);
            poll_input(&mut env);
            vga::write_vga(&BACK_BUFFER);
        }
        // ループ速度の調整
        for _ in 0..5000 { unsafe { core::arch::asm!("nop"); } }
    }
}

unsafe fn poll_input(env: &mut impl QuantumEnv) {
    let status: u8;
    core::arch::asm!("in al, 0x64", out("al") status);
    if status & 0x01 != 0 {
        let sc: u8;
        core::arch::asm!("in al, 0x60", out("al") sc);
        match sc {
            0x1E => { let _ = Q_MGR.allocate(); } // 'A' key
            0x13 => { for i in 0..8 { let _ = Q_MGR.release(i); } } // 'R' key
            0x39 => { env.interfere(Q_MGR.total_allocated as usize, 0xAA); } // SPACE key
            _ => {}
        }
    }
}

unsafe fn render(env: &mut impl QuantumEnv) {
    vga::print_buffer(&mut BACK_BUFFER, b"ITO-OS v0.2.0 [HIGH-FIDELITY]", 0, 0, 0x1B);
    
    vga::print_buffer(&mut BACK_BUFFER, b"ENTROPY:", 2, 2, 0x07);
    let entropy = env.get_entropy();
    vga::draw_num(&mut BACK_BUFFER, entropy, 2, 15, 0x0E);
    
    let ent_bar = (entropy / 500).min(20) as u8;
    vga::draw_bar(&mut BACK_BUFFER, 3, 2, ent_bar, 20, 0x40, 0x70);

    for i in 0..8 {
        let row = 6 + i;
        let slot = &Q_MGR.slots[i];
        vga::print_buffer(&mut BACK_BUFFER, b"UNIT", row, 2, 0x08);
        vga::draw_num(&mut BACK_BUFFER, i as u64, row, 7, 0x08);
        
        if slot.is_busy {
            vga::print_buffer(&mut BACK_BUFFER, b"[ACTIVE]", row, 10, 0x0A);
            vga::draw_bar(&mut BACK_BUFFER, row, 19, (slot.coherence / 10).min(10), 10, 0x20, 0x08);
        } else {
            vga::print_buffer(&mut BACK_BUFFER, b"[IDLE   ]", row, 10, 0x07);
            // バーの残像を消去
            vga::print_buffer(&mut BACK_BUFFER, b"          ", row, 19, 0x00);
        }
    }
    vga::print_buffer(&mut BACK_BUFFER, b"CTRL: [A] ALLOC  [R] RESET  [SPACE] INTERFERE", 24, 2, 0x08);
}

unsafe fn handle_panic(_err: QuantumError) -> ! {
    vga::clear_buffer(&mut BACK_BUFFER, 0x4F);
    vga::print_buffer(&mut BACK_BUFFER, b" !!! CRITICAL QUANTUM COLLAPSE !!! ", 10, 20, 0x1F);
    vga::write_vga(&BACK_BUFFER);
    loop { core::arch::asm!("hlt"); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[cfg(test)]
pub fn test_runner(_tests: &[&dyn Fn()]) {}