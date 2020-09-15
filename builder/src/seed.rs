use std::cell::RefCell;

pub struct Seed(RefCell<u64>);

impl Default for Seed {
    fn default() -> Self {
        Self(RefCell::new(0))
    }
}

impl Seed {
    pub fn generate(&self) -> u64 {
        let mut seed = self.0.borrow_mut();
        *seed += 1;
        *seed
    }
}
