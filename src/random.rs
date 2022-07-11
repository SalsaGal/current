use std::time::SystemTime;

/// A random noise function which is used to get random
/// but reproducable numbers.
pub struct Noise {
    pub seed: u32,
    pub index: u32,
}

impl Noise {
    /// Make a new noise generator with the seed set to the current unix time.
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        Self { seed, index: 0 }
    }

    pub fn from_seed(seed: u32) -> Self {
        Self { seed, index: 0 }
    }

    pub fn get(&self, index: u32) -> u32 {
        index.wrapping_add(479001599)
            ^ index.wrapping_pow(5)
            ^ self.seed.rotate_left(7)
            ^ index.rotate_right(1 + (self.seed % 3))
            ^ self.seed.rotate_right(index % self.seed)
    }
}

impl Iterator for Noise {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.index == u32::MAX {
            None
        } else {
            self.index += 1;
            Some(self.get(self.index))
        }
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}
