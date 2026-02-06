//! Kernel Resource Management v0.2.0

#[derive(Copy, Clone, Debug)]
pub enum QuantumError {
    ResourceExhausted,
    InvalidSlotAccess,
    DecoherenceCollapse,
}

#[derive(Copy, Clone, Debug)]
pub struct QuantumResource {
    pub id: u8,
    pub is_busy: bool,
    pub coherence: u8,
}

pub struct QuantumResourceManager {
    pub slots: [QuantumResource; 8],
    pub total_allocated: u8,
}

impl QuantumResourceManager {
    pub const fn new() -> Self {
        let mut slots = [QuantumResource { id: 0, is_busy: false, coherence: 100 }; 8];
        let mut i = 0;
        while i < 8 {
            slots[i].id = i as u8;
            i += 1;
        }
        Self { slots, total_allocated: 0 }
    }

    pub fn allocate(&mut self) -> Result<u8, QuantumError> {
        for slot in self.slots.iter_mut() {
            if !slot.is_busy {
                slot.is_busy = true;
                slot.coherence = 100;
                self.total_allocated += 1;
                return Ok(slot.id);
            }
        }
        Err(QuantumError::ResourceExhausted)
    }
    
    pub fn release(&mut self, id: u8) -> Result<(), QuantumError> {
        if id >= 8 { return Err(QuantumError::InvalidSlotAccess); }
        if self.slots[id as usize].is_busy {
            self.slots[id as usize].is_busy = false;
            self.total_allocated -= 1;
        }
        Ok(())
    }

    pub fn check_stability(&self, entropy: u64) -> Result<(), QuantumError> {
        if entropy > 9000 {
            return Err(QuantumError::DecoherenceCollapse);
        }
        Ok(())
    }

    pub fn decay_all(&mut self, entropy: u64) {
        let base_loss = (entropy / 500) as u8;
        for slot in self.slots.iter_mut() {
            if slot.is_busy {
                slot.coherence = slot.coherence.saturating_sub(base_loss);
            }
        }
    }
}