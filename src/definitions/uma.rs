use crate::circuit_prices::{RAM_PERMUTATION_COST_IN_ERGS, VM_CYCLE_COST_IN_ERGS};

use super::*;
use ethereum_types::U256;

pub const MAX_OFFSET_TO_DEREF_LOW_U32: u32 = ((1u64 << 32) - 33) as u32;

// Maximum offset which can be dereferenced. Formally we could dereference
// exactly 1<<32 - 32, but it would trigger extra checks and overflows, and in practice
// ergs cost to grow memory to that limit is beyond what can be used
pub const MAX_OFFSET_TO_DEREF: U256 = U256([MAX_OFFSET_TO_DEREF_LOW_U32 as u64, 0, 0, 0]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum UMAOpcode {
    HeapRead = 0,
    HeapWrite,
    AuxHeapRead,
    AuxHeapWrite,
    FatPointerRead,
}

pub const UMA_INCREMENT_FLAG_IDX: usize = 0;

impl OpcodeVariantProps for UMAOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            UMAOpcode::HeapRead,
            UMAOpcode::HeapWrite,
            UMAOpcode::AuxHeapRead,
            UMAOpcode::AuxHeapWrite,
            UMAOpcode::FatPointerRead,
        ]
    }

    fn max_variant_idx_for_version(version: ISAVersion) -> usize {
        match version {
            ISAVersion(0) => UMAOpcode::FatPointerRead.variant_index(),
            ISAVersion(1) => UMAOpcode::FatPointerRead.variant_index(),
            _ => unimplemented!(),
        }
    }

    fn minimal_version(&self) -> ISAVersion {
        match self {
            _ => ALL_ISA_VERSIONS[0],
        }
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == UMAOpcode::HeapRead.variant_index() => Some(UMAOpcode::HeapRead),
            i if i == UMAOpcode::HeapWrite.variant_index() => Some(UMAOpcode::HeapWrite),
            i if i == UMAOpcode::AuxHeapRead.variant_index() => Some(UMAOpcode::AuxHeapRead),
            i if i == UMAOpcode::AuxHeapWrite.variant_index() => Some(UMAOpcode::AuxHeapWrite),
            i if i == UMAOpcode::FatPointerRead.variant_index() => Some(UMAOpcode::FatPointerRead),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        match self {
            UMAOpcode::AuxHeapWrite | UMAOpcode::HeapWrite => {
                // 5 RAM permutations, because: 1 to read opcode + 2 reads + 2 writes.
                // 2 reads and 2 writes are needed because unaligned access is implemented with
                // aligned queries
                2 * VM_CYCLE_COST_IN_ERGS + 5 * RAM_PERMUTATION_COST_IN_ERGS
            }
            UMAOpcode::HeapRead | UMAOpcode::AuxHeapRead | UMAOpcode::FatPointerRead => {
                // 5 RAM permutations, because: 1 to read opcode + 2 reads.
                // 2 reads are needed because unaligned access is implemented with aligned queries
                VM_CYCLE_COST_IN_ERGS + 3 * RAM_PERMUTATION_COST_IN_ERGS
            }
        }
    }
}

impl OpcodeProps for UMAOpcode {
    fn name(&self) -> &'static str {
        "UMA opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 0)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 0)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, version: ISAVersion) -> usize {
        match version {
            ISAVersion(0) => UMAOpcode::FatPointerRead.variant_index(),
            ISAVersion(1) => UMAOpcode::FatPointerRead.variant_index(),
            _ => unimplemented!(),
        }
    }
    fn input_operands(&self, version: ISAVersion) -> Vec<Operand> {
        match version {
            ISAVersion(0) => match self {
                UMAOpcode::HeapWrite | UMAOpcode::AuxHeapWrite => {
                    vec![Operand::RegOnly, Operand::RegOnly]
                }
                _ => vec![Operand::RegOnly],
            },
            ISAVersion(1) => {
                // we allow imm on the inputs for heap access for offsets
                match self {
                    UMAOpcode::HeapWrite | UMAOpcode::AuxHeapWrite => vec![
                        Operand::RegOrImm(RegOrImmFlags::UseRegOnly),
                        Operand::RegOnly,
                    ],
                    UMAOpcode::HeapRead | UMAOpcode::AuxHeapRead => {
                        vec![Operand::RegOrImm(RegOrImmFlags::UseRegOnly)]
                    }
                    UMAOpcode::FatPointerRead => {
                        vec![Operand::RegOnly]
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            UMAOpcode::HeapWrite | UMAOpcode::AuxHeapWrite => vec![Operand::RegOnly],
            _ => vec![Operand::RegOnly, Operand::RegOnly],
        }
    }
    fn requires_kernel_mode(&self) -> bool {
        false
    }
    fn can_be_used_in_static_context(&self) -> bool {
        true
    }
}
