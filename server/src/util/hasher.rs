use std::hash::{BuildHasher, Hasher};

#[derive(Clone, Default)]
pub struct NoopHasherU32 {
    hash: u64,
}

impl BuildHasher for NoopHasherU32 {
    type Hasher = NoopHasherU32;

    fn build_hasher(&self) -> Self::Hasher {
        NoopHasherU32::default()
    }
}

impl Hasher for NoopHasherU32 {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        // Key is supposed to be hash already. we don't want to hash the hash.
        self.hash = u32::from_ne_bytes(bytes.try_into().unwrap()) as u64;
    }
}
