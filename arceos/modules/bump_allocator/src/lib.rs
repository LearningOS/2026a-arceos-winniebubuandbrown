#![no_std]
use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::{alloc::Layout, ptr::NonNull};

pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    count: usize,
}
impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        assert!(SIZE.is_power_of_two());
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            count: 0,
        }
    }
}
impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        let end = start.checked_add(size).expect("memory range overflow") & !(SIZE - 1);
        let start = start
            .max(1)
            .checked_add(SIZE - 1)
            .expect("alignment overflow")
            & !(SIZE - 1);
        assert!(start <= end, "memory region too small");
        *self = Self {
            start,
            end,
            b_pos: start,
            p_pos: end,
            count: 0,
        };
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if size == 0 || start % SIZE != 0 || size % SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let end = start.checked_add(size).ok_or(AllocError::InvalidParam)?;
        if start < self.end && end > self.start {
            return Err(AllocError::MemoryOverlap);
        }
        if start == self.end && self.p_pos == self.end {
            self.end = end;
            self.p_pos = end;
        } else if end == self.start && self.count == 0 {
            self.start = start;
            self.b_pos = start;
        } else {
            return Err(AllocError::InvalidParam);
        }
        Ok(())
    }
}
impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let pos = self
            .b_pos
            .checked_add(layout.align() - 1)
            .ok_or(AllocError::NoMemory)?
            & !(layout.align() - 1);
        let end = pos
            .checked_add(layout.size().max(1))
            .ok_or(AllocError::NoMemory)?;
        if end > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        let ptr = NonNull::new(pos as *mut u8).ok_or(AllocError::NoMemory)?;
        let count = self.count.checked_add(1).ok_or(AllocError::NoMemory)?;
        self.b_pos = end;
        self.count = count;
        Ok(ptr)
    }
    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        assert!(self.count > 0, "deallocation without allocation");
        self.count -= 1;
        if self.count == 0 {
            self.b_pos = self.start;
        }
    }
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }
    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }
    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}
impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if num_pages == 0 || !align_pow2.is_power_of_two() || align_pow2 < SIZE {
            return Err(AllocError::InvalidParam);
        }
        let bytes = num_pages.checked_mul(SIZE).ok_or(AllocError::NoMemory)?;
        let pos = self.p_pos.checked_sub(bytes).ok_or(AllocError::NoMemory)? & !(align_pow2 - 1);
        if pos < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        self.p_pos = pos;
        Ok(pos)
    }
    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {}
    fn total_pages(&self) -> usize {
        (self.end - self.start) / SIZE
    }
    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / SIZE
    }
    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / SIZE
    }
}

/*#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        todo!()
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        todo!()
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        todo!()
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        todo!()
    }

    fn available_bytes(&self) -> usize {
        todo!()
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        todo!()
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}*/
