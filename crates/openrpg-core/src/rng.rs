use std::ops::RangeInclusive;

use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct DeterministicRng {
    inner: ChaCha8Rng,
}

impl DeterministicRng {
    pub fn seeded(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    pub fn range_u32(&mut self, range: RangeInclusive<u32>) -> u32 {
        self.inner.random_range(range)
    }
}
