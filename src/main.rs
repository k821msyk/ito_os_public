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

// Global system state / グローバルな管理リソース
static mut BACK_BUFFER: [u8; 80 * 25 * 2] = [0; 80 * 25 * 2];
static mut Q_MGR: QuantumResourceManager = QuantumResourceManager::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut env = VirtualQuantumHAL::new();
    
    unsafe {
        vga::clear_buffer(&mut BACK_BUFFER, 0x00);
    }

    loop {
        unsafe {
            // 1. Physical Environment Update
            env.step();
            Q_MGR.decay_all(env.get_entropy());
            
            // 2. Kernel Stability Monitoring
            if let Err(e) = Q_MGR.check_stability(env.get_entropy()) {
                handle_panic(e);
            }

            // 3. UI Rendering
            render(&mut env);

            // 4. Input Polling
            poll_input(&mut env);

            // 5. Final Output to VGA
            vga::write_vga(&BACK_BUFFER);
        }
        // CPU Wait Loop
        for _ in 0..10000 { unsafe { core::arch::asm!("nop"); } }
    }
}

/// Poll keyboard input / 入力監視
unsafe fn poll_input(env: &mut impl QuantumEnv) {
    let status: u8;
    core::arch::asm!("in al, 0x64", out("al") status);
    if status & 0x01 != 0 {
        let sc: u8;
        core::arch::asm!("in al, 0x60", out("al") sc);
        match sc {
            0x1E => { let _ = Q_MGR.allocate(); } // [A] key
            0x13 => { for i in 0..8 { let _ = Q_MGR.release(i); } } // [R] key
            0x39 => { env.interfere(0, 0xFF); } // [SPACE] key
            _ => {}
        }
    }
}

/// Rendering loop / 描画ロジック
unsafe fn render(env: &mut impl QuantumEnv) {
    // Version updated to v0.1.0
    vga::print_buffer(&mut BACK_BUFFER, b"ITO-OS v0.1.0 [QUANTUM-CORE]", 0, 0, 0x1F);
    
    vga::print_buffer(&mut BACK_BUFFER, b"ENTROPY:", 2, 2, 0x07);
    let ent = (env.get_entropy() / 250).min(20) as u8;
    let ent_color = if ent > 15 { 0x40 } else { 0x20 };
    vga::draw_bar(&mut BACK_BUFFER, 2, 11, ent, 20, ent_color, 0x70);

    for i in 0..8 {
        let row = 4 + i;
        let slot = &Q_MGR.slots[i];
        vga::print_buffer(&mut BACK_BUFFER, b"SLOT", row, 2, 0x08);
        vga::draw_num(&mut BACK_BUFFER, i as u64, row, 7, 0x08);
        
        if slot.is_busy {
            vga::print_buffer(&mut BACK_BUFFER, b"ACTIVE", row, 10, 0x0A);
            vga::draw_bar(&mut BACK_BUFFER, row, 18, (slot.coherence / 10).min(10), 10, 0x20, 0x00);
        } else {
            vga::print_buffer(&mut BACK_BUFFER, b"IDLE   ", row, 10, 0x08);
            vga::print_buffer(&mut BACK_BUFFER, b"          ", row, 18, 0x00);
        }
    }
    vga::print_buffer(&mut BACK_BUFFER, b"CONTROLS: [A] ALLOC  [R] RESET  [SPACE] INTERFERE", 24, 2, 0x08);
}

unsafe fn handle_panic(err: QuantumError) -> ! {
    vga::clear_buffer(&mut BACK_BUFFER, 0x17);
    vga::print_buffer(&mut BACK_BUFFER, b" KERNEL PANIC: QUANTUM_COLLAPSE ", 10, 24, 0x1F);
    let msg: &[u8] = match err {
        QuantumError::DecoherenceCollapse => b"ERROR: CRITICAL_ENTROPY_OVERFLOW" as &[u8],
        _ => b"ERROR: UNKNOWN_FAULT" as &[u8],
    };
    vga::print_buffer(&mut BACK_BUFFER, msg, 12, 24, 0x1F);
    vga::write_vga(&BACK_BUFFER);
    loop { core::arch::asm!("hlt"); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
pub fn test_runner(_tests: &[&dyn Fn()]) {}