use tinyrand::{Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::config::PassConfig;

pub struct PassGenerator {
    valid_chars: Vec<char>,
    length: usize,
    rand: Box<dyn RandRange<usize>>,
}

impl PassGenerator {
    pub fn new(config: &PassConfig) -> Self {
        PassGenerator {
            valid_chars: config.valid_letters(),
            length: config.length,
            rand: Box::new(StdRand::seed(ClockSeed::default().next_u64())),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut password: Vec<char> = Vec::with_capacity(self.length);

        for i in 0..self.length {
            let r_idx = self.rand.next_range(0..self.valid_chars.len());
            let letter = self.valid_chars[r_idx];
            let insert_idx = if i == 0 {
                0
            } else {
                self.rand.next_range(0..password.len())
            };
            password.insert(insert_idx, letter);
        }

        String::from_iter(password.iter())
    }
}

impl Iterator for PassGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}
