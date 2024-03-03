use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::{alloc::Layout, ptr::NonNull};
use talc::{ErrOnOom, Span, Talc};

pub struct YourNewAllocator {
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
        self.heap.extend(0, size); //感觉有问题，先试试
        Ok(())
    }
}

impl ByteAllocator for YourNewAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        assert!(layout.size() > 0);
        unsafe { self.talc.malloc(layout).map_err(|_| AllocError::NoMemory) }
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe { self.talc.free(pos, layout) };
    }

    fn total_bytes(&self) -> usize {
        self.heap.size()
    }

    fn used_bytes(&self) -> usize {
        // self.inner.stats_alloc_actual()
    }

    fn available_bytes(&self) -> usize {
        // self.inner.stats_total_bytes() - self.inner.stats_alloc_actual()
    }
}
