use std::fs::File;
use std::io::Write;

use zkevm_opcode_defs::system_params::{
    ERGS_PER_CIRCUIT, INITIAL_STORAGE_WRITE_PUBDATA_BYTES, MAX_PUBDATA_PER_BLOCK,
    MAX_TX_ERGS_LIMIT, REPEATED_STORAGE_WRITE_PUBDATA_BYTES,
};

/// The number of input "units" the corresponding circuits could take.
/// It is assumed that the actual capacity of the circuits
/// below is greater or equal to the values provided there.
/// Some margin is suggested to not conduct a reprice upon every minor prover change.
pub const CYCLES_PER_VM_SNAPSHOT: u32 = 23000;
pub const CYCLES_PER_RAM_PERMUTATION: u32 = 260000;
pub const CYCLES_PER_CODE_DECOMMITTER: u32 = 12100;
pub const CYCLES_PER_STORAGE_APPLICATION: u32 = 118;
pub const CYCLES_PER_KECCAK256_CIRCUIT: u32 = 2050;
pub const CYCLES_PER_SHA256_CIRCUIT: u32 = 11500;
pub const CYCLES_PER_ECRECOVER_CIRCUIT: u32 = 72;
pub const CYCLES_FOR_CODE_DECOMMITTER_SORTER: u32 = 192500;
pub const CYCLES_FOR_LOG_DEMUXER: u32 = 101500;
pub const CYCLES_FOR_STORAGE_SORTER: u32 = 79000;
pub const CYCLES_FOR_EVENTS_OR_L1_MESSAGES_SORTER: u32 = 88000;

/// This kinds of circuit will always remain single-instance
pub const LIMIT_FOR_L1_MESSAGES_MERKLIZER: u32 = 512;
pub const LIMIT_FOR_INITIAL_WRITES_PUBDATA_HASHER: u32 = 4600;
pub const LIMIT_FOR_REPEATED_WRITES_PUBDATA_HASHER: u32 = 7400;

/// Returns ceil(a/b)
const fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn ergs_constant(name: &str, value: u32) -> String {
    format!("pub const {}: u32 = {};\n", name, value)
}

fn comment(comment: &str) -> String {
    format!("\n// {}\n", comment)
}

fn save_circiut_prices(prices: String, filepath: &str) {
    let file_content = vec![
        "// This file is auto-generated, do not edit it manually\n".to_owned(),
        "// Any changes to this file require system upgrade!\n\n".to_owned(),
        prices,
    ]
    .concat();
    let mut f = File::create(filepath).expect("Unable to create file");
    f.write_all(file_content.as_bytes())
        .expect("Unable to write data");
}

fn main() {
    let max_possible_initial_bytes_circuit_wise =
        LIMIT_FOR_INITIAL_WRITES_PUBDATA_HASHER * (INITIAL_STORAGE_WRITE_PUBDATA_BYTES as u32);
    let min_price_for_initial_pubdata_write =
        if MAX_PUBDATA_PER_BLOCK <= max_possible_initial_bytes_circuit_wise {
            // The cost of pubdata alone would not let the users to run out of this circuit
            // before the end of the transaction. The users can safely pay nothing for the DDoS security.
            0
        } else {
            ceil_div(MAX_TX_ERGS_LIMIT, LIMIT_FOR_INITIAL_WRITES_PUBDATA_HASHER)
        };

    let max_possible_repeated_bytes_circuit_wise =
        LIMIT_FOR_REPEATED_WRITES_PUBDATA_HASHER * (REPEATED_STORAGE_WRITE_PUBDATA_BYTES as u32);
    let min_price_for_repeated_pubdata_write =
        if MAX_PUBDATA_PER_BLOCK <= max_possible_repeated_bytes_circuit_wise {
            // The cost of pubdata alone would not let the users to run out of this circuit
            // before the end of the transaction. The users can safely pay nothing for the DDoS security.
            0
        } else {
            ceil_div(MAX_TX_ERGS_LIMIT, LIMIT_FOR_REPEATED_WRITES_PUBDATA_HASHER)
        };

    let circuit_prices = vec![
        ergs_constant(
            "VM_CYCLE_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_VM_SNAPSHOT),
        ),
        ergs_constant(
            "RAM_PERMUTATION_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_RAM_PERMUTATION),
        ),
        ergs_constant(
            "CODE_DECOMMITMENT_COST_PER_WORD_IN_ERGS",
            // Each round of decommitter outputs 64 bytes of the code,
            // while the user will pay for each word.
            ceil_div(ERGS_PER_CIRCUIT, 2 * CYCLES_PER_CODE_DECOMMITTER),
        ),
        ergs_constant(
            "STORAGE_APPLICATION_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_STORAGE_APPLICATION),
        ),
        ergs_constant(
            "CODE_DECOMMITTER_SORTER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_FOR_CODE_DECOMMITTER_SORTER),
        ),
        ergs_constant(
            "LOG_DEMUXER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_FOR_LOG_DEMUXER),
        ),
        ergs_constant(
            "STORAGE_SORTER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_FOR_STORAGE_SORTER),
        ),
        ergs_constant(
            "EVENTS_OR_L1_MESSAGES_SORTER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_FOR_EVENTS_OR_L1_MESSAGES_SORTER),
        ),
        ergs_constant(
            "INITIAL_WRITES_PUBDATA_HASHER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, LIMIT_FOR_INITIAL_WRITES_PUBDATA_HASHER),
        ),
        ergs_constant(
            "REPEATED_WRITES_PUBDATA_HASHER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, LIMIT_FOR_REPEATED_WRITES_PUBDATA_HASHER),
        ),
        ergs_constant(
            "CODE_DECOMMITMENT_SORTER_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_FOR_CODE_DECOMMITTER_SORTER)
        ),
        comment("The following circuits are single-instance and so the provided prices are just minimal prices to preserve DDoS safety"),
        ergs_constant(
            "L1_MESSAGE_MIN_COST_IN_ERGS",
            ceil_div(MAX_TX_ERGS_LIMIT, LIMIT_FOR_L1_MESSAGES_MERKLIZER),
        ),
        ergs_constant(
            "INITIAL_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS",
            min_price_for_initial_pubdata_write
        ),
        ergs_constant(
            "REPEATED_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS",
            min_price_for_repeated_pubdata_write
        ),
        comment("Equals to max(INITIAL_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS, REPEATED_WRITES_PUBDATA_HASHER_MIN_COST_IN_ERGS)"),
        ergs_constant(
            "STORAGE_WRITE_HASHER_MIN_COST_IN_ERGS",
            std::cmp::max(min_price_for_initial_pubdata_write, min_price_for_repeated_pubdata_write)
        ),
        comment("The following constants should not be used in the VM directly, but only in Solidity wrappers"),
        ergs_constant(
            "KECCAK256_CIRCUIT_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_KECCAK256_CIRCUIT),
        ),
        ergs_constant(
            "SHA256_CIRCUIT_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_SHA256_CIRCUIT),
        ),
        ergs_constant(
            "ECRECOVER_CIRCUIT_COST_IN_ERGS",
            ceil_div(ERGS_PER_CIRCUIT, CYCLES_PER_ECRECOVER_CIRCUIT),
        ),
    ]
    .concat();

    save_circiut_prices(circuit_prices, "src/circuit_prices.rs");
}
