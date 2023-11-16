use super::*;

// Note: offsets and length params below can be byte or word, or in general
// callee's interpreted
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrecompileCallABI {
    pub input_memory_offset: u32,
    pub input_memory_length: u32,
    pub output_memory_offset: u32,
    pub output_memory_length: u32,
    pub memory_page_to_read: u32,
    pub memory_page_to_write: u32,
    pub precompile_interpreted_data: u64,
}

impl PrecompileCallABI {
    pub const fn from_u256(raw_value: U256) -> Self {
        let raw = raw_value.0;
        let input_memory_offset = raw[0] as u32;
        let input_memory_length = (raw[0] >> 32) as u32;
        let output_memory_offset = raw[1] as u32;
        let output_memory_length = (raw[1] >> 32) as u32;
        let memory_page_to_read = raw[2] as u32;
        let memory_page_to_write = (raw[2] >> 32) as u32;
        let precompile_interpreted_data = raw[3];

        Self {
            input_memory_offset,
            input_memory_length,
            output_memory_offset,
            output_memory_length,
            memory_page_to_read,
            memory_page_to_write,
            precompile_interpreted_data,
        }
    }

    pub const fn to_u256(self) -> U256 {
        let mut result = U256::zero();
        result.0[0] = (self.input_memory_offset as u64) | ((self.input_memory_length as u64) << 32);
        result.0[1] =
            (self.output_memory_offset as u64) | ((self.output_memory_length as u64) << 32);
        result.0[2] =
            (self.memory_page_to_read as u64) | ((self.memory_page_to_write as u64) << 32);
        result.0[3] = self.precompile_interpreted_data;

        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrecompileAuxData {
    pub extra_ergs_cost: u32,
    pub extra_pubdata_cost: u32,
}

impl PrecompileAuxData {
    pub const fn from_u256(raw_value: U256) -> Self {
        let raw = raw_value.0;
        let extra_ergs_cost = raw[0] as u32;
        let extra_pubdata_cost = (raw[0] >> 32) as u32;

        Self {
            extra_ergs_cost,
            extra_pubdata_cost,
        }
    }

    pub const fn to_u256(self) -> U256 {
        let mut result = U256::zero();
        result.0[0] = (self.extra_ergs_cost as u64) | ((self.extra_pubdata_cost as u64) << 32);

        result
    }
}
