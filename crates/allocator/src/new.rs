use talc::{ErrOnOom, Span, Talc};

use crate::{AllocError, AllocResult, BaseAllocator, PageAllocator};

struct YourNewAllocator {
    talc: Talc<ErrOnOom>,
    heap: Span,
}

impl YourNewAllocator {
    pub const fn new() -> Self {
        Self {
            talc: Talc::new(ErrOnOom),
            heap: Span::empty(),
        }
    }
}

impl BaseAllocator for YourNewAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.heap = unsafe {
            self.talc
                .claim(Span::new(start as *mut u8, (start + size) as *mut u8))
                .unwrap()
        };
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        assert!(!self.heap.is_empty());
        self.heap.extend(0, size); //感觉有问题，先试试其他
        Ok(())
    }
}
