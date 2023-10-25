use crate::decoding::{EncodingModeProduction, VmEncodingMode};
use ethereum_types::U256;

pub const fn split_as_u4(value: u8) -> (u8, u8) {
    (value & ((1u8 << 4) - 1), value >> 4)
}

pub const fn merge_u4(low: u8, high: u8) -> u8 {
    debug_assert!(low < 16);
    debug_assert!(high < 16);
    low | (high << 4)
}

/// Internally we use versioned hash by our convensions
pub fn bytecode_to_code_hash(bytecode_words: &[[u8; 32]]) -> Result<[u8; 32], ()> {
    bytecode_to_code_hash_for_mode::<8, EncodingModeProduction>(bytecode_words)
}

/// Internally we use versioned hash by our convensions
pub fn bytecode_to_code_hash_for_mode<const N: usize, E: VmEncodingMode<N>>(
    bytecode_words: &[[u8; 32]],
) -> Result<[u8; 32], ()> {
    // bytecode should have an odd number of 32-byte words for ease of use of SHA256 round function
    if bytecode_words.len() % 2 != 1 {
        return Err(());
    }

    // limit of the bytecode length can be different in different modes
    use crate::decoding::AllowedPcOrImm;

    if bytecode_words.len() as u64 > E::PcOrImm::max().as_u64() {
        return Err(());
    }

    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    for w in bytecode_words.iter() {
        hasher.update(&w);
    }

    let result = hasher.finalize();

    let mut output = [0u8; 32];
    output[..].copy_from_slice(&result.as_slice());

    use crate::{ContractCodeSha256, VersionedHashGeneric};

    // this is invalid for modes other than production, but not that important
    // because testing mode doesn't do strict decommittments

    let versioned_hash =
        VersionedHashGeneric::<ContractCodeSha256>::from_digest_and_preimage_num_words(
            output,
            bytecode_words.len() as u16,
        );
    // this will place all the byte markers, lengths, etc

    let versioned_hash_bytes = versioned_hash.serialize().ok_or(())?;

    Ok(versioned_hash_bytes)
}

/// Erase start and page number from a fat pointer. To be used in the case of a fat pointer
/// being passed to an opcode which shouldn't receive one.
pub fn erase_fat_pointer_metadata(ptr: &mut U256) {
    // Memory page is at 1<<32 - 1<<64, start is at 1<<64 - 1<<96
    // We also need to preserve the top 128 bits of the value
    ptr.0[0] &= 0x00000000_ffffffffu64;
    ptr.0[1] &= 0xffffffff_00000000u64;
}
