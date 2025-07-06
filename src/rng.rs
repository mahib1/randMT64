use crate::util::{get_time, wrap_number, MAGIC};

pub struct MersenneTwister64 {
    mt: [u64; NN],
    index: usize,
}

const NN: usize = 312;
const MM: usize = 156;
const MATRIX_A64: u64 = 0xB5026F5AA96619E9;
const UM64: u64 = 0xFFFFFFFF80000000;
const LM64: u64 = 0x7FFFFFFF;

impl MersenneTwister64 {
    pub fn new(seed: u64) -> Self {
        let mut mt = [0u64; NN];
        mt[0] = seed;
        for i in 1..NN {
            mt[i] = 6364136223846793005u64
                .wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        Self { mt, index: NN }
    }

    fn twist(&mut self) {
        for i in 0..NN {
            let x = (self.mt[i] & UM64) + (self.mt[(i + 1) % NN] & LM64);
            let mut x_a = x >> 1;
            if x % 2 != 0 {
                x_a ^= MATRIX_A64;
            }
            self.mt[i] = self.mt[(i + MM) % NN] ^ x_a;
        }
        self.index = 0;
    }

    pub fn next_u64(&mut self) -> u64 {
        if self.index >= NN {
            self.twist();
        }

        let mut y = self.mt[self.index];
        y ^= (y >> 29) & 0x5555555555555555;
        y ^= (y << 17) & 0x71D67FFFEDA60000;
        y ^= (y << 37) & 0xFFF7EEE000000000;
        y ^= y >> 43;

        self.index += 1;
        y
    }

    pub fn next_u128(&mut self) -> u128 {
        ((self.next_u64() as u128) << 64) | (self.next_u64() as u128)
    }
}

pub struct Random {
    start: u128,
    end: u128,
}

impl Random {
    pub fn new(s: u128, e: u128) -> Self {
        Self { start: s, end: e * MAGIC }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self { start: 0, end: MAGIC }
    }
}

pub fn rand(config: &Random) -> Result<f64, std::io::Error> {
    let end = config.end;
    let start = config.start;

    Ok(wrap_number(get_time(), start, end) as f64 / MAGIC as f64)
}
