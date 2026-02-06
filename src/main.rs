#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(custom_test_frameworks)]

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
            0x1E => { // 'A' key: ALLOC & HADAMARD
                if let Ok(id) = Q_MGR.allocate() {
                    env.reset_qubit(id as usize);
                }
            } 
            0x13 => { // 'R' key: RESET
                for i in 0..8 { let _ = Q_MGR.release(i); }
            }
            0x39 => { // SPACE: MEASURE first superposition unit
                for i in 0..8 {
                    if Q_MGR.slots[i].is_busy {
                        let q = env.get_qubit(i);
                        // aかbのどちらかが100%(256)でない場合は「未測定」とみなす
                        if q.a.re != 256 && q.b.re != 256 {
                             env.measure(i);
                             break;
                        }
                    }
                }
            } 
            _ => {}
        }
    }
}

unsafe fn render(env: &mut impl QuantumEnv) {
    vga::print_buffer(&mut BACK_BUFFER, b"ITO-OS v0.2.1 [QUANTUM CORE]", 0, 0, 0x1B);
    vga::print_buffer(&mut BACK_BUFFER, b"ENTROPY:", 2, 2, 0x07);
    vga::draw_num(&mut BACK_BUFFER, env.get_entropy(), 2, 15, 0x0E);
    
    for i in 0..8 {
        let row = 6 + i;
        let slot = &Q_MGR.slots[i];
        let q_state = env.get_qubit(i);
        
        vga::print_buffer(&mut BACK_BUFFER, b"UNIT", row, 2, 0x08);
        vga::draw_num(&mut BACK_BUFFER, i as u64, row, 7, 0x08);
        
        if slot.is_busy {
            let p1 = q_state.b.norm_sq();
            let total = (q_state.a.norm_sq() + p1).max(1);
            let bar_val = ((p1 * 10) / total) as u8;

            vga::print_buffer(&mut BACK_BUFFER, b"|psi>", row, 10, 0x0B);
            vga::draw_bar(&mut BACK_BUFFER, row, 19, bar_val, 10, 0x20, 0x08);
            
            // 状態表示の分岐をより厳密に
            if q_state.a.re == 256 {
                vga::print_buffer(&mut BACK_BUFFER, b"[ |0> ]", row, 32, 0x0A);
            } else if q_state.b.re == 256 {
                vga::print_buffer(&mut BACK_BUFFER, b"[ |1> ]", row, 32, 0x0C);
            } else {
                vga::print_buffer(&mut BACK_BUFFER, b"[SUPER]", row, 32, 0x0D);
            }
        } else {
            vga::print_buffer(&mut BACK_BUFFER, b"[IDLE   ]", row, 10, 0x07);
            vga::print_buffer(&mut BACK_BUFFER, b"          ", row, 19, 0x00);
            vga::print_buffer(&mut BACK_BUFFER, b"       ", row, 32, 0x00);
        }
    }
    vga::print_buffer(&mut BACK_BUFFER, b"CTRL: [A] ALLOC+H  [R] RESET  [SPACE] MEASURE", 24, 2, 0x08);
}

unsafe fn handle_panic(_err: QuantumError) -> ! {
    vga::clear_buffer(&mut BACK_BUFFER, 0x4F);
    vga::print_buffer(&mut BACK_BUFFER, b" !!! CRITICAL QUANTUM COLLAPSE !!! ", 10, 20, 0x1F);
    vga::write_vga(&BACK_BUFFER);
    loop { core::arch::asm!("hlt"); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }