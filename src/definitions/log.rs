use crate::{
    circuit_prices::{
        EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS, L1_MESSAGE_MIN_COST_IN_ERGS,
        LOG_DEMUXER_COST_IN_ERGS, RAM_PERMUTATION_COST_IN_ERGS, STORAGE_SORTER_COST_IN_ERGS,
        VM_CYCLE_COST_IN_ERGS,
    },
    system_params::MIN_STORAGE_WRITE_COST,
};

use super::*;

pub const FIRST_MESSAGE_FLAG_IDX: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LogOpcode {
    StorageRead = 0,
    StorageWrite,
    ToL1Message,
    Event,
    PrecompileCall,
}

impl OpcodeVariantProps for LogOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            LogOpcode::StorageRead,
            LogOpcode::StorageWrite,
            LogOpcode::ToL1Message,
            LogOpcode::Event,
            LogOpcode::PrecompileCall,
        ]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        LogOpcode::PrecompileCall.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == LogOpcode::StorageRead.variant_index() => Some(LogOpcode::StorageRead),
            i if i == LogOpcode::StorageWrite.variant_index() => Some(LogOpcode::StorageWrite),
            i if i == LogOpcode::ToL1Message.variant_index() => Some(LogOpcode::ToL1Message),
            i if i == LogOpcode::Event.variant_index() => Some(LogOpcode::Event),
            i if i == LogOpcode::PrecompileCall.variant_index() => Some(LogOpcode::PrecompileCall),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        match self {
            LogOpcode::StorageRead => {
                STORAGE_READ_IO_PRICE
                    + VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + LOG_DEMUXER_COST_IN_ERGS
                    + STORAGE_SORTER_COST_IN_ERGS
            }
            // If the write was not initial, the user will be refunded
            LogOpcode::StorageWrite => {
                let intrinsic = STORAGE_WRITE_IO_PRICE
                    + 2 * VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * STORAGE_SORTER_COST_IN_ERGS;

                std::cmp::max(intrinsic, MIN_STORAGE_WRITE_COST)
            }
            // Note, that the `L1_MESSAGE_MIN_COST_IN_ERGS` is only needed for DDoS protection
            LogOpcode::ToL1Message => {
                let intrinsic_cost = L1_MESSAGE_IO_PRICE
                    + 2 * VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS;
                std::cmp::max(intrinsic_cost, L1_MESSAGE_MIN_COST_IN_ERGS)
            }
            LogOpcode::Event => {
                EVENT_IO_PRICE
                    + 2 * VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS
            }
            LogOpcode::PrecompileCall => {
                VM_CYCLE_COST_IN_ERGS + RAM_PERMUTATION_COST_IN_ERGS + LOG_DEMUXER_COST_IN_ERGS
            }
        }
    }
}

impl OpcodeProps for LogOpcode {
    fn name(&self) -> &'static str {
        "Log opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) | ISAVersion(1) => {
                vec![
                    // Storage read
                    OpcodeVariantData {
                        variant_idx: LogOpcode::StorageRead.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                    // Storage write
                    OpcodeVariantData {
                        variant_idx: LogOpcode::StorageWrite.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                    // L1 message
                    OpcodeVariantData {
                        variant_idx: LogOpcode::ToL1Message.variant_index(),
                        num_non_exclusive_flags: 1, // can be "initial"
                        num_used_immediates: 0,
                    },
                    // Event
                    OpcodeVariantData {
                        variant_idx: LogOpcode::Event.variant_index(),
                        num_non_exclusive_flags: 1, // can be initial
                        num_used_immediates: 0,
                    },
                    // Precompile calls
                    OpcodeVariantData {
                        variant_idx: LogOpcode::PrecompileCall.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                ]
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        LogOpcode::PrecompileCall.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            LogOpcode::StorageWrite | LogOpcode::Event | LogOpcode::ToL1Message => {
                vec![Operand::RegOnly, Operand::RegOnly]
            }
            LogOpcode::StorageRead => vec![Operand::RegOnly],
            LogOpcode::PrecompileCall => vec![Operand::RegOnly],
        }
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            LogOpcode::StorageWrite | LogOpcode::Event | LogOpcode::ToL1Message => vec![],
            LogOpcode::StorageRead => vec![Operand::RegOnly],
            LogOpcode::PrecompileCall => vec![Operand::RegOnly],
        }
    }
    fn requires_kernel_mode(&self) -> bool {
        match self {
            LogOpcode::Event | LogOpcode::ToL1Message | LogOpcode::PrecompileCall => true,
            _ => false,
        }
    }
    fn can_be_used_in_static_context(&self) -> bool {
        match self {
            LogOpcode::StorageWrite | LogOpcode::Event | LogOpcode::ToL1Message => false,
            _ => true,
        }
    }
}
