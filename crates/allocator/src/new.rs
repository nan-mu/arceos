use talc::{ErrOnOom, Talc};

use crate::{AllocError, AllocResult, BaseAllocator, PageAllocator};

struct YourNewAllocator {
    talc: Talc<ErrOnOom>,
}

impl YourNewAllocator {
    pub const fn new() -> Self {
        Self {
            talc: Talc::new(ErrOnOom),
        }
    }
}

impl BaseAllocator for YourNewAllocator {
    fn init(&mut self, start: usize, size: usize) {
        let memory = talc::Span::new(*start, acme)
        self.talc.claim(memory);
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        Ok(())
    }
}
