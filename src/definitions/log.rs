use crate::{
    circuit_prices::{
        EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS, LOG_DEMUXER_COST_IN_ERGS,
        RAM_PERMUTATION_COST_IN_ERGS, STORAGE_SORTER_COST_IN_ERGS, VM_CYCLE_COST_IN_ERGS,
    },
    system_params::{STORAGE_ACCESS_COLD_READ_COST, STORAGE_ACCESS_COLD_WRITE_COST},
};

use self::{
    circuit_prices::{CODE_DECOMMITMENT_SORTER_COST_IN_ERGS, TRANSIENT_STORE_CHECKER_COST_IN_ERGS},
    system_params::{STORAGE_ACCESS_WARM_READ_COST, STORAGE_ACCESS_WARM_WRITE_COST},
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
    Decommit,
    TransientStorageRead,
    TransientStorageWrite,
}

impl OpcodeVariantProps for LogOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            LogOpcode::StorageRead,
            LogOpcode::StorageWrite,
            LogOpcode::ToL1Message,
            LogOpcode::Event,
            LogOpcode::PrecompileCall,
            LogOpcode::Decommit,
            LogOpcode::TransientStorageRead,
            LogOpcode::TransientStorageWrite,
        ]
    }

    fn max_variant_idx_for_version(version: ISAVersion) -> usize {
        match version {
            ISAVersion(0) | ISAVersion(1) => LogOpcode::PrecompileCall.variant_index(),
            ISAVersion(2) => LogOpcode::TransientStorageWrite.variant_index(),
            _ => unreachable!(),
        }
    }

    fn minimal_version(&self) -> ISAVersion {
        match self {
            LogOpcode::StorageRead
            | LogOpcode::StorageWrite
            | LogOpcode::ToL1Message
            | LogOpcode::Event
            | LogOpcode::PrecompileCall => ISAVersion(0),
            LogOpcode::Decommit
            | LogOpcode::TransientStorageRead
            | LogOpcode::TransientStorageWrite => ISAVersion(2),
        }
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, version: &ISAVersion) -> Option<Self> {
        match version {
            ISAVersion(0) | ISAVersion(1) => match index {
                i if i == LogOpcode::StorageRead.variant_index() => Some(LogOpcode::StorageRead),
                i if i == LogOpcode::StorageWrite.variant_index() => Some(LogOpcode::StorageWrite),
                i if i == LogOpcode::ToL1Message.variant_index() => Some(LogOpcode::ToL1Message),
                i if i == LogOpcode::Event.variant_index() => Some(LogOpcode::Event),
                i if i == LogOpcode::PrecompileCall.variant_index() => {
                    Some(LogOpcode::PrecompileCall)
                }
                _ => None,
            },
            ISAVersion(2) => match index {
                i if i == LogOpcode::StorageRead.variant_index() => Some(LogOpcode::StorageRead),
                i if i == LogOpcode::StorageWrite.variant_index() => Some(LogOpcode::StorageWrite),
                i if i == LogOpcode::ToL1Message.variant_index() => Some(LogOpcode::ToL1Message),
                i if i == LogOpcode::Event.variant_index() => Some(LogOpcode::Event),
                i if i == LogOpcode::PrecompileCall.variant_index() => {
                    Some(LogOpcode::PrecompileCall)
                }
                i if i == LogOpcode::Decommit.variant_index() => Some(LogOpcode::Decommit),
                i if i == LogOpcode::TransientStorageRead.variant_index() => {
                    Some(LogOpcode::TransientStorageRead)
                }
                i if i == LogOpcode::TransientStorageWrite.variant_index() => {
                    Some(LogOpcode::TransientStorageWrite)
                }
                _ => None,
            },
            _ => unreachable!(),
        }
    }

    fn ergs_price(&self) -> u32 {
        match self {
            LogOpcode::StorageRead => {
                VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + LOG_DEMUXER_COST_IN_ERGS
                    + STORAGE_SORTER_COST_IN_ERGS
                    + STORAGE_ACCESS_COLD_READ_COST
            }
            // If the write was not initial, the user will be refunded
            LogOpcode::StorageWrite => {
                VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * STORAGE_SORTER_COST_IN_ERGS
                    + STORAGE_ACCESS_COLD_WRITE_COST
            }
            // Note, that the `L1_MESSAGE_MIN_COST_IN_ERGS` is only needed for DDoS protection
            LogOpcode::ToL1Message => {
                L1_MESSAGE_IO_PRICE
                    + VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS
            }
            LogOpcode::Event => {
                EVENT_IO_PRICE
                    + VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS
            }
            LogOpcode::PrecompileCall => {
                VM_CYCLE_COST_IN_ERGS + RAM_PERMUTATION_COST_IN_ERGS + LOG_DEMUXER_COST_IN_ERGS
            }
            LogOpcode::Decommit => {
                VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + CODE_DECOMMITMENT_SORTER_COST_IN_ERGS
            }
            LogOpcode::TransientStorageRead => {
                VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + LOG_DEMUXER_COST_IN_ERGS
                    + TRANSIENT_STORE_CHECKER_COST_IN_ERGS
                    // The pricing for transient operations is the same as for the warm storage ones, due
                    // to the need of handling reverts.
                    + STORAGE_ACCESS_WARM_READ_COST
            }
            LogOpcode::TransientStorageWrite => {
                VM_CYCLE_COST_IN_ERGS
                    + RAM_PERMUTATION_COST_IN_ERGS
                    + 2 * LOG_DEMUXER_COST_IN_ERGS
                    + 2 * TRANSIENT_STORE_CHECKER_COST_IN_ERGS
                    // The pricing for transient operations is the same as for the warm storage ones, due
                    // to the need of handling reverts.
                    + STORAGE_ACCESS_WARM_WRITE_COST
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
            ISAVersion(2) => {
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
                    // Decommit request
                    OpcodeVariantData {
                        variant_idx: LogOpcode::Decommit.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                    // Transient read
                    OpcodeVariantData {
                        variant_idx: LogOpcode::TransientStorageRead.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                    // Transient write
                    OpcodeVariantData {
                        variant_idx: LogOpcode::TransientStorageWrite.variant_index(),
                        num_non_exclusive_flags: 0,
                        num_used_immediates: 0,
                    },
                ]
            }

            _ => unimplemented!(),
        }
    }

    fn max_variant_idx(&self, version: ISAVersion) -> usize {
        match version {
            ISAVersion(0) | ISAVersion(1) => LogOpcode::PrecompileCall.variant_index(),
            ISAVersion(2) => LogOpcode::TransientStorageWrite.variant_index(),
            _ => unreachable!(),
        }
    }

    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            LogOpcode::StorageWrite
            | LogOpcode::Event
            | LogOpcode::ToL1Message
            | LogOpcode::PrecompileCall
            | LogOpcode::Decommit
            | LogOpcode::TransientStorageWrite => {
                vec![Operand::RegOnly, Operand::RegOnly]
            }
            LogOpcode::StorageRead | Self::TransientStorageRead => vec![Operand::RegOnly],
        }
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            LogOpcode::StorageWrite
            | LogOpcode::Event
            | LogOpcode::ToL1Message
            | LogOpcode::TransientStorageWrite => vec![],
            LogOpcode::StorageRead | LogOpcode::TransientStorageRead => vec![Operand::RegOnly],
            LogOpcode::PrecompileCall => vec![Operand::RegOnly],
            LogOpcode::Decommit => vec![Operand::RegOnly],
        }
    }
    fn requires_kernel_mode(&self) -> bool {
        match self {
            LogOpcode::Event
            | LogOpcode::ToL1Message
            | LogOpcode::PrecompileCall
            | LogOpcode::Decommit => true,
            _ => false,
        }
    }
    fn can_be_used_in_static_context(&self) -> bool {
        match self {
            LogOpcode::StorageWrite
            | LogOpcode::Event
            | LogOpcode::ToL1Message
            | LogOpcode::TransientStorageWrite => false,
            _ => true,
        }
    }

    fn src0_can_be_pointer(&self) -> bool {
        false
    }

    fn src1_can_be_pointer(&self) -> bool {
        false
    }
}
