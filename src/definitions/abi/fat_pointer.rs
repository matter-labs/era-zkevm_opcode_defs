use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FatPointer {
    pub offset: u32,      // offset relative to `start`
    pub memory_page: u32, // memory page where slice is located
    pub start: u32,       // absolute start of the slice
    pub length: u32,      // length of the slice
}

bitflags::bitflags! {
    pub struct FatPointerValidationException: u64 {
        const OFFSET_IS_NOT_ZERO_WHEN_EXPECTED = 1u64 << 0;
        const DEREF_BEYOND_HEAP_RANGE = 1u64 << 1;
    }
}

impl FatPointer {
    pub const FAT_POINTER_PACKED_WIDTH: u32 = 128;

    // formally empty, for internal uses mainly
    pub const fn empty() -> Self {
        Self {
            length: 0,
            start: 0,
            memory_page: 0,
            offset: 0,
        }
    }

    pub const fn from_u256(value: U256) -> Self {
        let raw_value = value.0;
        let offset = raw_value[0] as u32;
        let memory_page = (raw_value[0] >> 32) as u32;

        let start = raw_value[1] as u32;
        let length = (raw_value[1] >> 32) as u32;

        Self {
            offset,
            memory_page,
            start,
            length,
        }
    }

    pub fn validate(self, is_fresh: bool) -> FatPointerValidationException {
        let mut exceptions = FatPointerValidationException::empty();

        // we have 2 invariants:
        // fresh one has `offset` == 0
        if is_fresh && self.offset != 0 {
            exceptions.set(
                FatPointerValidationException::OFFSET_IS_NOT_ZERO_WHEN_EXPECTED,
                true,
            );
        }
        // start + length doesn't overflow
        let (_, of) = self.start.overflowing_add(self.length);
        if of {
            exceptions.set(FatPointerValidationException::DEREF_BEYOND_HEAP_RANGE, true);
        }

        exceptions
    }

    /// IMPORTANT: when we create a pointer in VM using checked routine
    /// we know that `start` and `length` form a good combination in terms of no overflows
    /// and spanning only addressable region, so we only need to check that `offset` < `length`
    pub const fn validate_in_bounds(&self) -> bool {
        let in_bounds = self.offset < self.length;

        in_bounds
    }

    /// We allow to pass empty (offset == length) slices in Far call / Ret
    pub const fn validate_as_slice(&self) -> bool {
        let is_valid_slice = self.offset <= self.length;

        is_valid_slice
    }

    /// special case when we supply empty slice, that is formally NOT addressable
    pub const fn is_trivial(&self) -> bool {
        self.length == 0 && self.offset == 0
    }

    pub const fn to_u256(self) -> U256 {
        let mut result = U256::zero();
        result.0[0] = (self.offset as u64) | ((self.memory_page as u64) << 32);

        result.0[1] = (self.start as u64) | ((self.length as u64) << 32);

        result
    }
}
