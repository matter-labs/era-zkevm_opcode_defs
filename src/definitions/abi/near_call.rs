use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NearCallABI {
    pub ergs_passed: u32,
}

impl NearCallABI {
    pub const fn from_u256(raw_value: U256) -> Self {
        let raw = raw_value.0;
        let ergs_passed = raw[0] as u32;

        Self { ergs_passed }
    }
}
