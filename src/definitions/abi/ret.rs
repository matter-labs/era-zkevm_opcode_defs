use super::*;

pub const RET_FORWARDING_MODE_BYTE_IDX: usize = 28;

const _: () = if RET_FORWARDING_MODE_BYTE_IDX != super::far_call::FAR_CALL_FORWARDING_MODE_BYTE_IDX
{
    panic!()
} else {
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RetForwardPageType {
    UseHeap = 0,
    ForwardFatPointer,
    UseAuxHeap,
}

impl RetForwardPageType {
    pub const fn from_u8(value: u8) -> Self {
        // use inline consts when stable
        match value {
            0 => RetForwardPageType::UseHeap,
            1 => RetForwardPageType::ForwardFatPointer,
            2 => RetForwardPageType::UseAuxHeap,
            _ => RetForwardPageType::UseHeap, // default
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RetABI {
    pub memory_quasi_fat_pointer: FatPointer,
    pub page_forwarding_mode: RetForwardPageType,
}

impl RetABI {
    pub const fn from_u256(raw_value: U256) -> Self {
        let quasi_fat_pointer = FatPointer::from_u256(raw_value);

        let raw = raw_value.0;
        let extra_data = (raw[3] >> 32) as u32;
        let [forwarding_byte, _, _, _] = extra_data.to_le_bytes();
        let page_forwarding_mode = RetForwardPageType::from_u8(forwarding_byte);

        Self {
            memory_quasi_fat_pointer: quasi_fat_pointer,
            page_forwarding_mode,
        }
    }
}
