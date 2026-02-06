//! Quantum State Vector Simulation Layer v0.2.1 (Revised)

pub mod physics {
    pub const MAX_ENTROPY: u64 = 10000;
    pub const COOLING_RATE: u64 = 5;
}

/// 16-bit固定小数点による複素数表現 (Q8.8形式)
/// 1.0 = 256
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    pub re: i16,
    pub im: i16,
}

impl Complex {
    pub const fn new(re: i16, im: i16) -> Self { Self { re, im } }
    
    /// 振幅の二乗（存在確率）を計算
    pub fn norm_sq(&self) -> u32 {
        ((self.re as i32 * self.re as i32) + (self.im as i32 * self.im as i32)) as u32
    }
}

/// 1キュービットの状態ベクトル: |ψ> = a|0> + b|1>
#[derive(Copy, Clone, Debug)]
pub struct QubitState {
    pub a: Complex,
    pub b: Complex,
}

pub trait QuantumEnv {
    fn step(&mut self);
    fn measure(&mut self, id: usize) -> u8;
    fn reset_qubit(&mut self, id: usize);
    fn get_entropy(&self) -> u64;
    fn get_qubit(&self, id: usize) -> QubitState;
}

pub struct VirtualQuantumHAL {
    qubits: [QubitState; 8],
    entropy: u64,
    rng_state: u64,
}

impl VirtualQuantumHAL {
    pub fn new() -> Self {
        let zero_state = QubitState {
            a: Complex::new(256, 0),
            b: Complex::new(0, 0),
        };
        Self {
            qubits: [zero_state; 8],
            entropy: 0,
            rng_state: 0xACE1,
        }
    }

    fn next_rng(&mut self) -> u64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        self.rng_state
    }
}

impl QuantumEnv for VirtualQuantumHAL {
    fn step(&mut self) {
        self.entropy = self.entropy.saturating_sub(physics::COOLING_RATE);
    }

    /// ユニットを重ね合わせ状態 |+> に初期化
    fn reset_qubit(&mut self, id: usize) {
        if id < 8 {
            // 1/√2 ≈ 181/256
            self.qubits[id] = QubitState {
                a: Complex::new(181, 0),
                b: Complex::new(181, 0),
            };
        }
    }

    /// 確率的測定と波動関数の収縮
    fn measure(&mut self, id: usize) -> u8 {
        if id >= 8 { return 0; }
        
        let p0 = self.qubits[id].a.norm_sq();
        let p1 = self.qubits[id].b.norm_sq();
        let total = (p0 + p1) as u64;
        
        if total == 0 { return 0; }

        let r = self.next_rng() % total;
        
        if r < p0 as u64 {
            self.qubits[id].a = Complex::new(256, 0);
            self.qubits[id].b = Complex::new(0, 0);
            0
        } else {
            self.qubits[id].a = Complex::new(0, 0);
            self.qubits[id].b = Complex::new(256, 0);
            1
        }
    }

    fn get_entropy(&self) -> u64 { self.entropy }
    fn get_qubit(&self, id: usize) -> QubitState { self.qubits[id] }
}