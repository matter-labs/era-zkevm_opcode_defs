pub const REGISTERS_COUNT: usize = 15;

pub mod decoding;
pub mod definitions;
pub mod imm_mem_modifiers;
pub mod opcode;
pub mod system_params;
pub mod utils;

pub mod circuit_prices;

use std::collections::HashMap;

use circuit_prices::CODE_DECOMMITMENT_COST_PER_WORD_IN_ERGS;
use circuit_prices::CODE_DECOMMITTER_SORTER_COST_IN_ERGS;
use circuit_prices::RAM_PERMUTATION_COST_IN_ERGS;
use circuit_prices::VM_CYCLE_COST_IN_ERGS;

pub use bitflags;
pub use blake2;
pub use ethereum_types;
pub use k256;
pub use sha2;
pub use sha3;

pub use self::definitions::*;
pub use self::imm_mem_modifiers::*;
pub use self::opcode::*;
pub use self::utils::*;

pub use self::system_params::{
    ADDRESS_ACCOUNT_CODE_STORAGE, ADDRESS_BOOTLOADER, ADDRESS_CONTRACT_DEPLOYER, ADDRESS_ECRECOVER,
    ADDRESS_ETH_TOKEN, ADDRESS_EVENT_WRITER, ADDRESS_FORCE_DEPLOYER, ADDRESS_IDENTITY,
    ADDRESS_IMMUTABLE_SIMULATOR, ADDRESS_KECCAK256, ADDRESS_KNOWN_CODES_STORAGE,
    ADDRESS_L1_MESSENGER, ADDRESS_MSG_VALUE, ADDRESS_NONCE_HOLDER, ADDRESS_RIPEMD160,
    ADDRESS_SHA256, ADDRESS_SYSTEM_CONTEXT, ADDRESS_UNRESTRICTED_SPACE,
};

use lazy_static::lazy_static;

use crate::decoding::VariantMonotonicNumber;

pub const OPCODES_TABLE_WIDTH: usize = 11;
pub const CONDITIONAL_BITS_SHIFT: usize = 13;
pub const MEMORY_GROWTH_ERGS_PER_BYTE: u32 = 1;

const _: () = if MEMORY_GROWTH_ERGS_PER_BYTE != 1 {
    panic!()
};

pub const VARIANT_AND_CONDITION_ENCODING_BITS: usize = 16;

pub const REGISTER_INDEX_ENCODING_BITS: usize = 4;

pub const SRC_REGS_SHIFT: u32 = 16;
pub const DST_REGS_SHIFT: u32 = 24;

// flattened bits the exclusively (mostly) encode all the opcode properties
pub const OPCODE_TYPE_BITS: usize = NUM_OPCODES;

pub const OPCODE_INPUT_VARIANT_FLAGS: usize = 6;
pub const OPCODE_OUTPUT_VARIANT_FLAGS: usize = 4;

// aux flags for resolution of exceptions
pub const KERNEL_MODE_FLAG_BITS: usize = 1;
pub const CAN_BE_USED_IN_STATIC_CONTEXT_FLAG_BITS: usize = 1;
pub const EXPLICIT_PANIC_FLAG_BITS: usize = 1;

pub const KERNER_MODE_FLAG_IDX: usize = 0;
pub const CAN_BE_USED_IN_STATIC_CONTEXT_FLAG_IDX: usize = 1;
pub const EXPLICIT_PANIC_FLAG_IDX: usize = 2;

const WIDTH_MULTIPLE: usize = 16;

pub const INITIAL_SP_ON_FAR_CALL: u64 = 0;
pub const UNMAPPED_PAGE: u32 = 0;

pub const BOOTLOADER_BASE_PAGE: u32 = 8;
pub const BOOTLOADER_CODE_PAGE: u32 = BOOTLOADER_BASE_PAGE;
pub const BOOTLOADER_CALLDATA_PAGE: u32 = BOOTLOADER_BASE_PAGE - 1; // some convention
pub const BOOTLOADER_STACK_PAGE: u32 = BOOTLOADER_BASE_PAGE + 1;
pub const BOOTLOADER_HEAP_PAGE: u32 = BOOTLOADER_BASE_PAGE + 2;
pub const BOOTLOADER_AUX_HEAP_PAGE: u32 = BOOTLOADER_BASE_PAGE + 3;

pub const NEW_MEMORY_PAGES_PER_FAR_CALL: u32 = 8;
pub const STARTING_TIMESTAMP: u32 = 1024;
pub const STARTING_BASE_PAGE: u32 = 2048;
pub const TIME_DELTA_PER_CYCLE: u32 = 4;
pub const MAX_PENDING_CYCLES: usize = 1;

pub const LOG2_NUM_ADDRESSABLE_HEAP_BYTES: u32 = 24;

pub(crate) const NUM_NON_EXCLUSIVE_FLAGS: usize = 2;

pub fn total_description_bits_for_version(version: ISAVersion) -> usize {
    let total = OPCODE_TYPE_BITS
        + max_num_variants_for_version(version)
        + max_num_flags_for_version(version)
        + OPCODE_INPUT_VARIANT_FLAGS
        + OPCODE_OUTPUT_VARIANT_FLAGS;

    total
}

pub fn total_description_bits_rounded_for_version(version: ISAVersion) -> usize {
    let mut total = OPCODE_TYPE_BITS
        + max_num_variants_for_version(version)
        + max_num_flags_for_version(version)
        + OPCODE_INPUT_VARIANT_FLAGS
        + OPCODE_OUTPUT_VARIANT_FLAGS;
    if total % WIDTH_MULTIPLE != 0 {
        total += WIDTH_MULTIPLE - total % WIDTH_MULTIPLE;
    }

    total
}

pub const TOTAL_AUX_BITS: usize =
    KERNEL_MODE_FLAG_BITS + CAN_BE_USED_IN_STATIC_CONTEXT_FLAG_BITS + EXPLICIT_PANIC_FLAG_BITS;

pub fn total_opcode_description_and_aux_bits_for_version(version: ISAVersion) -> usize {
    total_description_bits_rounded_for_version(version) + TOTAL_AUX_BITS
}

pub const DEFAULT_ISA_VERSION: ISAVersion = ISAVersion(1);

lazy_static! {
    pub static ref OPCODES_TABLE: [OpcodeVariant; 1 << OPCODES_TABLE_WIDTH] = {
        synthesize_opcode_decoding_tables(OPCODES_TABLE_WIDTH, DEFAULT_ISA_VERSION).try_into().unwrap()
    };

    pub static ref OPCODES_PROPS_INTEGER_BITMASKS: [u64; 1 << OPCODES_TABLE_WIDTH] = {
        synthesize_bit_decomposition_table(&*OPCODES_TABLE, DEFAULT_ISA_VERSION).try_into().unwrap()
    };

    pub static ref OPCODE_TO_MONOTONIC_INDEX_NUMBER_MAP: HashMap<OpcodeVariant, VariantMonotonicNumber> = {
        let mut result = HashMap::<OpcodeVariant, VariantMonotonicNumber>::new();
        for (idx, el) in OPCODES_TABLE.iter().enumerate() {
            if let Some(existing) = result.get(el) {
                let usize_index = (*existing).into_usize();
                assert_eq!(OPCODES_TABLE[usize_index], INVALID_OPCODE_VARIANT);
            } else {
                let _ = result.insert(*el, VariantMonotonicNumber::from_usize(idx));
            }
        }

        result
    };

    pub static ref OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP: HashMap<OpcodeVariant, usize> = {
        let mut result = HashMap::new();
        for (idx, el) in OPCODES_TABLE.iter().enumerate() {
            if let Some(existing) = result.get(el) {
                assert_eq!(OPCODES_TABLE[*existing], INVALID_OPCODE_VARIANT);
            } else {
                let _ = result.insert(*el, idx);
            }
        }

        result
    };

    pub static ref NOP_OPCODE_VARIANT: OpcodeVariant = {
        let variant = OpcodeVariant{
            opcode: Opcode::Nop(NopOpcode),
            src0_operand_type: Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            dst0_operand_type: Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            flags: [false; NUM_NON_EXCLUSIVE_FLAGS],
        };
        assert!(OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP.contains_key(&variant));

        variant
    };

    pub static ref NOP_BITSPREAD_U64: u64 = {
        let index = OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP[&NOP_OPCODE_VARIANT];
        let bitspread = OPCODES_PROPS_INTEGER_BITMASKS[index];

        bitspread
    };

    pub static ref PANIC_OPCODE_VARIANT: OpcodeVariant = {
        let variant = OpcodeVariant{
            opcode: Opcode::Ret(RetOpcode::Panic),
            src0_operand_type: Operand::RegOnly,
            dst0_operand_type: Operand::RegOnly,
            flags: [false; NUM_NON_EXCLUSIVE_FLAGS],
        };

        assert!(OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP.contains_key(&variant));

        variant
    };

    pub static ref PANIC_BITSPREAD_U64: u64 = {
        let index = OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP[&PANIC_OPCODE_VARIANT];
        let bitspread = OPCODES_PROPS_INTEGER_BITMASKS[index];

        bitspread
    };

    pub static ref OPCODE_PROTOTYPES: Vec<Box<dyn OpcodeProps>> = {
        all_opcodes()
    };

    pub static ref NUM_LOGICAL_OPCODES: usize = {
        OPCODE_PROTOTYPES.len()
    };

    pub static ref NUM_INPUT_VARIANTS: usize = {
        ImmMemHandlerFlags::num_src_variants()
    };

    pub static ref NUM_OUTPUT_VARIANTS: usize = {
        ImmMemHandlerFlags::num_dst_variants()
    };

    // Preliminary pricing
    pub static ref OPCODES_PRICES: [u32; 1 << OPCODES_TABLE_WIDTH] = {
        let mut result = Vec::with_capacity(1 << OPCODES_TABLE_WIDTH);
        for opcode in OPCODES_TABLE.iter() {
            let price = opcode.ergs_price();
            result.push(price)
        }

        result.try_into().unwrap()
    };
}

pub const INVALID_OPCODE_ERGS: u32 = u32::MAX; // will burn everything at once

// `RICH_ADDRESSING_OPCODE_ERGS` is for opcodes that can write
// their return value/read the input onto the stack and so take 1-2 RAM permutations more than
// an average opcode. Note, that while, in the worst case, a rich addressing may take 3 ram permutations
// (1 for reading the opcode, 1 for writing input value, 1 for writing output value), the
// 1 "reading of opcode" reads 4 sequential opcodes at the same time, so if we priced users by the worst case
// (VM_CYCLE_COST_IN_ERGS + 3 * RAM_PERMUTATION_COST_IN_ERGS), they would overpay too much, while in case of a DDoS
// attack, we would only overpay only 1.2x.
pub const RICH_ADDRESSING_OPCODE_ERGS: u32 =
    VM_CYCLE_COST_IN_ERGS + 2 * RAM_PERMUTATION_COST_IN_ERGS;
pub const AVERAGE_OPCODE_ERGS: u32 = VM_CYCLE_COST_IN_ERGS + RAM_PERMUTATION_COST_IN_ERGS;

/// The following prices are meant to take into account the I/O overhead for
/// these operations (i.e. state bloat that becomes with them)
pub const STORAGE_READ_IO_PRICE: u32 = 150;
pub const STORAGE_WRITE_IO_PRICE: u32 = 250;
pub const EVENT_IO_PRICE: u32 = 25;
pub const L1_MESSAGE_IO_PRICE: u32 = 100; // Extra for merklization

/// This variable is meant to represent the cost for creating a new item on callstack
pub const CALL_LIKE_ERGS_COST: u32 = 20;

pub const ERGS_PER_CODE_WORD_DECOMMITTMENT: u32 = CODE_DECOMMITMENT_COST_PER_WORD_IN_ERGS;

const _: () = if CODE_DECOMMITTER_SORTER_COST_IN_ERGS > u16::MAX as u32 {
    panic!()
};
