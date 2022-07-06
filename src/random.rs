pub struct Noise {
    pub seed: u32,
    pub index: u32,
}

impl Noise {
    pub fn from_seed(seed: u32) -> Self {
        Self {
            seed,
            index: 0,
        }
    }

    pub fn get(&self, index: u32) -> u32 {
        index.wrapping_add(479001599)
            ^ index.wrapping_pow(5)
            ^ index.rotate_left(7)
            ^ index.rotate_right(9)
            ^ self.seed.rotate_right(index % self.seed)
    }

    pub fn next(&mut self) -> u32 {
        self.index += 1;
        self.get(self.index)
    }
}
