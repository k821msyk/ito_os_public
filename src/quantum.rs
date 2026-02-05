//! Virtual Physics Layer / 仮想物理レイヤ

pub trait QuantumEnv {
    fn step(&mut self);
    fn interfere(&mut self, addr: usize, val: u8) -> bool;
    fn get_entropy(&self) -> u64;
}

pub struct VirtualQuantumHAL {
    cells: [u8; 1024],
    entropy: u64,
    tick: u64,
}

impl VirtualQuantumHAL {
    pub fn new() -> Self {
        Self { cells: [0; 1024], entropy: 0, tick: 0 }
    }
}

impl QuantumEnv for VirtualQuantumHAL {
    fn step(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        if self.tick % 5 == 0 {
            let idx = (self.tick as usize * 13) % 1024;
            if self.entropy > 1500 {
                self.cells[idx] = self.cells[idx].wrapping_add(1);
            }
        }
        self.entropy = self.entropy.saturating_sub(10);
    }

    fn interfere(&mut self, addr: usize, val: u8) -> bool {
        if addr >= 1024 { return false; }
        self.entropy = self.entropy.saturating_add(150);
        self.cells[addr] = val;
        true
    }

    fn get_entropy(&self) -> u64 {
        self.entropy
    }
}