use crate::circuit_prices::{
    CODE_DECOMMITMENT_SORTER_COST_IN_ERGS, RAM_PERMUTATION_COST_IN_ERGS,
    STORAGE_SORTER_COST_IN_ERGS, VM_CYCLE_COST_IN_ERGS,
};

use super::*;

// NOTE: registers are zero-enumerated
pub const CALL_IMPLICIT_CALLDATA_FAT_PTR_REGISTER: u8 = 0;
pub const CALL_IMPLICIT_CONSTRUCTOR_MARKER_REGISTER: u8 = 1;
pub const CALL_SYSTEM_ABI_REGISTERS: std::ops::Range<u8> = 2..12;
pub const CALL_RESERVED_RANGE: std::ops::Range<u8> = 12..14;
pub const CALL_IMPLICIT_PARAMETER_REG_IDX: u8 = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FarCallOpcode {
    Normal = 0,
    Delegate,
    Mimic,
}

pub const FAR_CALL_STATIC_FLAG_IDX: usize = 0;
pub const FAR_CALL_SHARD_FLAG_IDX: usize = 1;

impl OpcodeVariantProps for FarCallOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            FarCallOpcode::Normal,
            FarCallOpcode::Delegate,
            FarCallOpcode::Mimic,
        ]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        FarCallOpcode::Mimic.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == FarCallOpcode::Normal.variant_index() => Some(FarCallOpcode::Normal),
            i if i == FarCallOpcode::Delegate.variant_index() => Some(FarCallOpcode::Delegate),
            i if i == FarCallOpcode::Mimic.variant_index() => Some(FarCallOpcode::Mimic),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        2 * VM_CYCLE_COST_IN_ERGS
            + RAM_PERMUTATION_COST_IN_ERGS
            + STORAGE_READ_IO_PRICE
            + CALL_LIKE_ERGS_COST
            + STORAGE_SORTER_COST_IN_ERGS
            + CODE_DECOMMITMENT_SORTER_COST_IN_ERGS
    }
}

impl OpcodeProps for FarCallOpcode {
    fn name(&self) -> &'static str {
        "Far call opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 2, 1)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 2, 1)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        FarCallOpcode::Mimic.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![Operand::RegOnly, Operand::RegOnly]
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![]
    }
    fn requires_kernel_mode(&self) -> bool {
        match self {
            FarCallOpcode::Mimic => true,
            _ => false,
        }
    }
    fn can_be_used_in_static_context(&self) -> bool {
        true
    }
}
