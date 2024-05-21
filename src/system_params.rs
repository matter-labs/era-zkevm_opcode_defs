use crate::{
    circuit_prices::STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS, CALL_LIKE_ERGS_COST,
    ERGS_PER_CODE_WORD_DECOMMITTMENT,
};
use ethereum_types::Address;

pub const MAX_TX_ERGS_LIMIT: u32 = 80_000_000;

pub const VM_INITIAL_FRAME_ERGS: u32 = u32::MAX;

pub const EVM_SIMULATOR_STIPEND: u32 = 1u32 << 30;

/// How much a single circuit should cost in terms of ergs.
pub const ERGS_PER_CIRCUIT: u32 = 80000;

/// The callstack depth large enough to ensure that we will not exceed the stack limit
/// in any of the transactions. THe `+80` is an arbitrary value, meant to take into accounts
/// some of the internal near_calls that are performed by the bootloader which the user does
/// not pay directly to.
pub const VM_MAX_STACK_DEPTH: u32 = VM_INITIAL_FRAME_ERGS / CALL_LIKE_ERGS_COST + 80;

pub const INITIAL_FRAME_SUCCESSFUL_EXIT_PC: u16 = 0u16;
pub const INITIAL_FRAME_FORMAL_EH_LOCATION: u16 = u16::MAX;

const SYSTEM_CONTRACTS_OFFSET_ADDRESS: u16 = 1 << 15;

pub const KECCAK256_ROUND_FUNCTION_PRECOMPILE_ADDRESS: u16 = SYSTEM_CONTRACTS_OFFSET_ADDRESS + 0x10;
pub const SHA256_ROUND_FUNCTION_PRECOMPILE_ADDRESS: u16 = 0x02; // as in Ethereum
pub const ECRECOVER_INNER_FUNCTION_PRECOMPILE_ADDRESS: u16 = 0x01; // as in Ethereum
pub const SECP256R1_VERIFY_PRECOMPILE_ADDRESS: u16 = 0x100; // As in RIP7212: https://github.com/ethereum/RIPs/blob/master/RIPS/rip-7212.md

pub const MAX_PUBDATA_COST_PER_QUERY: i32 = 65;
pub const INITIAL_STORAGE_WRITE_PUBDATA_BYTES: usize = 64;
pub const REPEATED_STORAGE_WRITE_PUBDATA_BYTES: usize = 40;
pub const L1_MESSAGE_PUBDATA_BYTES: u32 = 1 + 1 + 2 + 20 + 32 + 32;

/// The maximal amount of public data in bytes that could be sent within an L1 batch
/// The limit that the Geth nodes impose is 128kb.
/// We leave 10kb margin for possible parameters.
pub const MAX_PUBDATA_PER_BLOCK: u32 = 110000;

pub const STORAGE_AUX_BYTE: u8 = 0;
pub const EVENT_AUX_BYTE: u8 = 1;
pub const L1_MESSAGE_AUX_BYTE: u8 = 2;
pub const PRECOMPILE_AUX_BYTE: u8 = 3;
pub const TRANSIENT_STORAGE_AUX_BYTE: u8 = 4;

use lazy_static::lazy_static;

pub const BOOTLOADER_FORMAL_ADDRESS_LOW: u16 = SYSTEM_CONTRACTS_OFFSET_ADDRESS + 0x01;
pub const DEPLOYER_SYSTEM_CONTRACT_ADDRESS_LOW: u16 = SYSTEM_CONTRACTS_OFFSET_ADDRESS + 0x02;

/// The unrestricted address space beginning.
pub const ADDRESS_UNRESTRICTED_SPACE: u64 = 1u64 << 16;

pub const ADDRESS_ECRECOVER: u16 = 0x0001;
pub const ADDRESS_SHA256: u16 = 0x0002;
pub const ADDRESS_RIPEMD160: u16 = 0x0003;
pub const ADDRESS_IDENTITY: u16 = 0x0004;
pub const ADDRESS_MODEXP: u16 = 0x0005;
pub const ADDRESS_ECADD: u16 = 0x0006;
pub const ADDRESS_ECMUL: u16 = 0x0007;
pub const ADDRESS_ECPAIRING: u16 = 0x0008;
pub const ADDRESS_BLAKE2F: u16 = 0x0009;
pub const ADDRESS_POINT_EVALUATION: u16 = 0x000A;

pub const ADDRESS_BOOTLOADER: u16 = 0x8001;
pub const ADDRESS_ACCOUNT_CODE_STORAGE: u16 = 0x8002;
pub const ADDRESS_NONCE_HOLDER: u16 = 0x8003;
pub const ADDRESS_KNOWN_CODES_STORAGE: u16 = 0x8004;
pub const ADDRESS_IMMUTABLE_SIMULATOR: u16 = 0x8005;
pub const ADDRESS_CONTRACT_DEPLOYER: u16 = 0x8006;
pub const ADDRESS_FORCE_DEPLOYER: u16 = 0x8007;
pub const ADDRESS_L1_MESSENGER: u16 = 0x8008;
pub const ADDRESS_MSG_VALUE: u16 = 0x8009;
pub const ADDRESS_ETH_TOKEN: u16 = 0x800A;
pub const ADDRESS_SYSTEM_CONTEXT: u16 = 0x800B;
pub const ADDRESS_BOOTLOADER_UTILITIES: u16 = 0x800C;
pub const ADDRESS_EVENT_WRITER: u16 = 0x800D;
pub const ADDRESS_KECCAK256: u16 = 0x8010;
pub const ADDRESS_CODE_ORACLE: u16 = 0x8011;

pub const BOOTLOADER_MAX_MEMORY: u32 = u32::MAX;
// 4 KB for new frames is "free"
pub const NEW_FRAME_MEMORY_STIPEND: u32 = 1u32 << 12;
// 2 MB for kernel frames, where we can be sure about the behavior.
// Note, that this number should high enough to allow any bytecode for `decommit` opcode.
pub const NEW_KERNEL_FRAME_MEMORY_STIPEND: u32 = 1u32 << 21;

pub const INTERNAL_ERGS_TO_VISIBLE_ERGS_CONVERSION_CONSTANT: u32 = 1;

/// `MsgValueSimulator` will automatically support decommitments to bytecodes of size up to 100k.
/// It will mean that if 0 gas was provided for the call, only `callee`s of size up to 100k could be called.
/// We supporting a larger value would lead to larger overhead for callers that do not provide 0 gas.
pub const MAX_AUTOMATICALLY_SUPPORTED_MSG_VALUE_BYTECODE: u32 = 100_000;
const _: () = assert!(MAX_AUTOMATICALLY_SUPPORTED_MSG_VALUE_BYTECODE % 32 == 0);
pub const DECOMMITMENT_MSG_VALUE_SIMULATOR_OVERHEAD: u32 =
    ERGS_PER_CODE_WORD_DECOMMITTMENT * MAX_AUTOMATICALLY_SUPPORTED_MSG_VALUE_BYTECODE / 32;

/// The amount of gas that is always retrived from the `caller` and passed to the `MsgValueSimulator` whenever it is called.
/// This value should be enough to cover the execution of the `MsgValueSimulator` itself and the decommitment of the callee's bytecode + pass at least 2300 gas.
/// This invariant is not easy to enforce within this crate, so `MsgValueSimulator` is expected to be well tested in the `era-contracts` repo.
pub const MSG_VALUE_SIMULATOR_ADDITIVE_COST: u32 =
    14500 + DECOMMITMENT_MSG_VALUE_SIMULATOR_OVERHEAD;

// std::cmp::max is not yet stabilized as const fn yet
const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

/// The minimum price in ergs that a storage write should cost in order to protect Ethereum's `.transfer / .send` function against reentrancy.
/// It is a 2300 constant used for 0-value `transfer/send` calls + 1 to make sure that within the call it is not possible to store anything.
pub const MIN_STORAGE_WRITE_PRICE_FOR_REENTRANCY_PROTECTION: u32 = 2300 + 1;

/// The minimal price in ergs the storage could cost to protect against reentrancy + take into account the usage of the single instance circuits.
pub const MIN_STORAGE_WRITE_COST: u32 = max(
    MIN_STORAGE_WRITE_PRICE_FOR_REENTRANCY_PROTECTION,
    STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS,
);

pub const STORAGE_ACCESS_COLD_READ_COST: u32 = 2000;
pub const STORAGE_ACCESS_COLD_WRITE_COST: u32 = max(MIN_STORAGE_WRITE_COST, 5500);

pub const STORAGE_ACCESS_WARM_READ_COST: u32 = 30;
pub const STORAGE_ACCESS_WARM_WRITE_COST: u32 = 60;

/// We currently ensure that that cost of each byte of pubdata computation-wise should be equal to at least 80 ergs.
/// This is needed to ensure that a call with bounded amount of gas could not published too much pubdata.
const _: () = assert!(STORAGE_ACCESS_COLD_WRITE_COST / (MAX_PUBDATA_COST_PER_QUERY as u32) >= 80);

lazy_static! {
    pub static ref BOOTLOADER_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(BOOTLOADER_FORMAL_ADDRESS_LOW as u64);
    pub static ref DEPLOYER_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(DEPLOYER_SYSTEM_CONTRACT_ADDRESS_LOW as u64);
    pub static ref NONCE_MANAGER_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(SYSTEM_CONTRACTS_OFFSET_ADDRESS as u64 + 0x03);
    pub static ref KNOWN_CODE_FACTORY_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(SYSTEM_CONTRACTS_OFFSET_ADDRESS as u64 + 0x04);
    pub static ref CODE_ORACLE_ADDRESS: Address =
        Address::from_low_u64_be(SYSTEM_CONTRACTS_OFFSET_ADDRESS as u64 + 0x12);
    pub static ref KECCAK256_ROUND_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(KECCAK256_ROUND_FUNCTION_PRECOMPILE_ADDRESS as u64);
    pub static ref SHA256_ROUND_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(SHA256_ROUND_FUNCTION_PRECOMPILE_ADDRESS as u64);
    pub static ref ECRECOVER_INNER_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(ECRECOVER_INNER_FUNCTION_PRECOMPILE_ADDRESS as u64);
    pub static ref SECP256R1_VERIFY_INNER_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(SECP256R1_VERIFY_PRECOMPILE_ADDRESS as u64);
}
