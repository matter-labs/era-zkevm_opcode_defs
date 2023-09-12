use crate::{circuit_prices::STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS, CALL_LIKE_ERGS_COST};
use ethereum_types::Address;

pub const MAX_TX_ERGS_LIMIT: u32 = 80_000_000;

pub const VM_INITIAL_FRAME_ERGS: u32 = u32::MAX;

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

pub const NUM_SPONGES: usize = 4;

use lazy_static::lazy_static;

pub const BOOTLOADER_FORMAL_ADDRESS_LOW: u16 = SYSTEM_CONTRACTS_OFFSET_ADDRESS + 0x01;
pub const DEPLOYER_SYSTEM_CONTRACT_ADDRESS_LOW: u16 = SYSTEM_CONTRACTS_OFFSET_ADDRESS + 0x02;

/// The unrestricted address space beginning.
pub const ADDRESS_UNRESTRICTED_SPACE: u64 = 1u64 << 16;

pub const ADDRESS_ECRECOVER: u16 = 0x0001;
pub const ADDRESS_SHA256: u16 = 0x0002;
pub const ADDRESS_RIPEMD160: u16 = 0x0003;
pub const ADDRESS_IDENTITY: u16 = 0x0004;

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

pub const BOOTLOADER_MAX_MEMORY: u32 = u32::MAX;
pub const NEW_FRAME_MEMORY_STIPEND: u32 = 1u32 << 10; // 1 KB for new frames is "free"

pub const MSG_VALUE_SIMULATOR_PUBDATA_BYTES_TO_PREPAY: u32 = 32 + 32 + 32 + 32;

/// 128k * 4 / 32 -- the maximal realistic smart contract size. NOTE this constant should be updated once a new
/// packing method is introduced or more than 128k of data is allowed to be sent on L1.
pub const DECOMMITMENT_MSG_VALUE_SIMULATOR_OVERHEAD: u32 = 64000;
pub const MSG_VALUE_SIMULATOR_ADDITIVE_COST: u32 =
    11500 + DECOMMITMENT_MSG_VALUE_SIMULATOR_OVERHEAD;

/// The minimum amount of ergs that should be spent by the user while using the MsgValueSimulator (even if
/// the user spends less funds, only the parent frame will receivet the refund)
pub const MSG_VALUE_SIMULATOR_MIN_USED_ERGS: u32 = 8000 + DECOMMITMENT_MSG_VALUE_SIMULATOR_OVERHEAD;

// std::cmp::max is not yet stabilized as const fn yet
const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

/// The minimum price in ergs that a storage write should cost in order to protect Ethereum's `.transfer / .send` function against reentrancy.
/// The first part of the expression is the stipend given by the MsgValueSimulator to the callee frame. The second part of the expression
/// is the 2300 constant used for 0-value `transfer/send` calls + 1 to make sure that within the call it is not possible to store anything.
pub const MIN_STORAGE_WRITE_PRICE_FOR_REENTRANCY_PROTECTION: u32 = max(
    MSG_VALUE_SIMULATOR_ADDITIVE_COST - MSG_VALUE_SIMULATOR_MIN_USED_ERGS + 1,
    2300 + 1,
);

/// The minimal price in ergs the storage could cost to protect against reentrancy + take into account the usage of the single instance circuits.
pub const MIN_STORAGE_WRITE_COST: u32 = max(
    MIN_STORAGE_WRITE_PRICE_FOR_REENTRANCY_PROTECTION,
    STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS,
);

lazy_static! {
    pub static ref BOOTLOADER_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(BOOTLOADER_FORMAL_ADDRESS_LOW as u64);
    pub static ref DEPLOYER_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(DEPLOYER_SYSTEM_CONTRACT_ADDRESS_LOW as u64);
    pub static ref NONCE_MANAGER_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(SYSTEM_CONTRACTS_OFFSET_ADDRESS as u64 + 0x03);
    pub static ref KNOWN_CODE_FACTORY_SYSTEM_CONTRACT_ADDRESS: Address =
        Address::from_low_u64_be(SYSTEM_CONTRACTS_OFFSET_ADDRESS as u64 + 0x04);
    pub static ref KECCAK256_ROUND_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(KECCAK256_ROUND_FUNCTION_PRECOMPILE_ADDRESS as u64);
    pub static ref SHA256_ROUND_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(SHA256_ROUND_FUNCTION_PRECOMPILE_ADDRESS as u64);
    pub static ref ECRECOVER_INNER_FUNCTION_PRECOMPILE_FORMAL_ADDRESS: Address =
        Address::from_low_u64_be(ECRECOVER_INNER_FUNCTION_PRECOMPILE_ADDRESS as u64);
}
