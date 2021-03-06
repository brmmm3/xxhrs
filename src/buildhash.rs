use crate::{
    entropy::EntropyPool,
    xxh3::{XXH3_128, XXH3_64},
    xxhash::{XXH32, XXH64},
};
use getrandom::getrandom;
use std::{default::Default, hash::BuildHasher};

/// xxhash 32 bit version. Generates a randomized seed using getrandom().
/// Because the associated hasher generates u32, this does not implement BuildHash,
/// however this features the same interface.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RandomStateXXH32 {
    pub seed: u32,
}

impl Default for RandomStateXXH32 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStateXXH32 {
    #[inline]
    pub fn new() -> Self {
        let mut seed = [0u8; 4];
        getrandom(&mut seed).unwrap();
        Self {
            seed: u32::from_ne_bytes(seed),
        }
    }

    #[inline]
    pub fn build_hasher(&self) -> XXH32 {
        XXH32::with_seed(self.seed)
    }
}

/// xxhash 64 bit version. Generates a randomized seed using getrandom().
/// Implements BuildHasher.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RandomStateXXH64 {
    pub seed: u64,
}

impl Default for RandomStateXXH64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStateXXH64 {
    pub fn new() -> Self {
        let mut seed = [0u8; 8];
        getrandom(&mut seed).unwrap();
        Self {
            seed: u64::from_ne_bytes(seed),
        }
    }
}

impl BuildHasher for RandomStateXXH64 {
    type Hasher = XXH64;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::with_seed(self.seed)
    }
}

/// xxh3 64 bit version. Generates a randomized seed using getrandom().
/// Implements BuildHasher.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RandomStateXXH3_64 {
    pub pool: EntropyPool,
}

impl Default for RandomStateXXH3_64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStateXXH3_64 {
    #[inline]
    pub fn new() -> Self {
        Self {
            pool: EntropyPool::randomize(),
        }
    }
}

impl BuildHasher for RandomStateXXH3_64 {
    type Hasher = XXH3_64<'static>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::with_entropy(&self.pool)
    }
}

/// xxh3 128 bit version. Generates a randomized seed using getrandom().
/// Because the associated hasher generates u32, this does not implement BuildHash,
/// however this features the same interface.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RandomStateXXH3_128 {
    pub pool: EntropyPool,
}

impl Default for RandomStateXXH3_128 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStateXXH3_128 {
    #[inline]
    pub fn new() -> Self {
        Self {
            pool: EntropyPool::randomize(),
        }
    }

    #[inline]
    pub fn build_hasher(&self) -> XXH3_128<'static> {
        XXH3_128::with_entropy(&self.pool)
    }
}
