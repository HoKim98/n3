use std::cell::RefCell;

pub struct Seed(RefCell<u64>);

impl Default for Seed {
    fn default() -> Self {
        Self(RefCell::new(1))
    }
}

impl Seed {
    pub fn generate(&self) -> u64 {
        self.alloc(1)
    }

    pub fn alloc(&self, len: u64) -> u64 {
        let mut seed = self.0.borrow_mut();
        let seed_origin = *seed;
        *seed += len;
        seed_origin
    }
}
