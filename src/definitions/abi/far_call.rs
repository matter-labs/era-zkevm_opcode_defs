use super::*;

pub const FAR_CALL_FORWARDING_MODE_BYTE_IDX: usize = 28;
pub const FAR_CALL_SHARD_ID_BYTE_IDX: usize = 29;
pub const FAR_CALL_CONSTRUCTOR_CALL_BYTE_IDX: usize = 30;
pub const FAR_CALL_SYSTEM_CALL_BYTE_IDX: usize = 31;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FarCallForwardPageType {
    UseHeap = 0,
    ForwardFatPointer,
    UseAuxHeap,
}

impl FarCallForwardPageType {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => FarCallForwardPageType::UseHeap,
            1 => FarCallForwardPageType::ForwardFatPointer,
            2 => FarCallForwardPageType::UseAuxHeap,
            _ => FarCallForwardPageType::UseHeap, // default
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FarCallABI {
    pub memory_quasi_fat_pointer: FatPointer,
    pub ergs_passed: u32,
    pub shard_id: u8,
    pub forwarding_mode: FarCallForwardPageType,
    pub constructor_call: bool,
    pub to_system: bool,
}

impl FarCallABI {
    pub const fn from_u256(raw_value: U256) -> Self {
        let quasi_fat_pointer = FatPointer::from_u256(raw_value);

        let raw = raw_value.0;

        let extra_data = raw[3];
        let ergs_passed = extra_data as u32;

        let extra_data = (extra_data >> 32) as u32;

        let [forwarding_byte, shard_id, constructor_call_byte, to_system_byte] =
            extra_data.to_le_bytes();

        let page_forwarding_mode = FarCallForwardPageType::from_u8(forwarding_byte);
        let constructor_call = constructor_call_byte != 0;
        let to_system = to_system_byte != 0;

        Self {
            memory_quasi_fat_pointer: quasi_fat_pointer,
            ergs_passed,
            shard_id,
            forwarding_mode: page_forwarding_mode,
            constructor_call,
            to_system,
        }
    }
}
