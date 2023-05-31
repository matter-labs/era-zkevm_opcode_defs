// This file is auto-generated, do not edit it manually
// Any changes to this file require system upgrade!

pub const VM_CYCLE_COST_IN_ERGS: u32 = 4;
pub const RAM_PERMUTATION_COST_IN_ERGS: u32 = 1;
pub const CODE_DECOMMITMENT_COST_PER_WORD_IN_ERGS: u32 = 4;
pub const STORAGE_APPLICATION_COST_IN_ERGS: u32 = 678;
pub const CODE_DECOMMITTER_SORTER_COST_IN_ERGS: u32 = 1;
pub const LOG_DEMUXER_COST_IN_ERGS: u32 = 1;
pub const STORAGE_SORTER_COST_IN_ERGS: u32 = 2;
pub const EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS: u32 = 1;
pub const INITIAL_WRITES_PUBDATA_HASHER_COST_IN_ERGS: u32 = 18;
pub const REPEATED_WRITES_PUBDATA_HASHER_COST_IN_ERGS: u32 = 11;
pub const CODE_DECOMMITMENT_SORTER_COST_IN_ERGS: u32 = 1;

// The following circuits are single-instance and so the provided prices are just minimal prices to preserve DDoS safety
pub const L1_MESSAGE_MIN_COST_IN_ERGS: u32 = 156250;
pub const INITIAL_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS: u32 = 0;
pub const REPEATED_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS: u32 = 0;

// Equals to max(INITIAL_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS, REPEATED_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS)
pub const STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS: u32 = 0;

// The following constants should not be used in the VM directly, but only in Solidity wrappers
pub const KECCAK256_CIRCUIT_COST_IN_ERGS: u32 = 40;
pub const SHA256_CIRCUIT_COST_IN_ERGS: u32 = 7;
pub const ECRECOVER_CIRCUIT_COST_IN_ERGS: u32 = 1112;
