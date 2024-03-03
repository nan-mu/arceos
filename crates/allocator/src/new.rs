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
        self.heap = unsafe {
            self.talc.extend(
                self.heap,
                Span::new(start as *mut u8, (start + size) as *mut u8),
            )
        };
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
        //这里有歧义，talc实现了一个free函数用于释放，实现了shrink用于收缩。看起来两者都实现了dealloc。不知道用哪个
    }

    fn total_bytes(&self) -> usize {
        self.heap.size()
    }

    fn used_bytes(&self) -> usize {
        unsafe { self.talc.get_allocated_span(self.heap).size() }
    }

    fn available_bytes(&self) -> usize {
        self.heap.size() - unsafe { self.talc.get_allocated_span(self.heap).size() }
    }
}
