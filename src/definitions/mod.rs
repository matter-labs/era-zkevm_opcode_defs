use std::collections::HashSet;

use crate::imm_mem_modifiers::*;

use super::*;

pub mod opcode_trait;
pub use self::opcode_trait::*;

pub mod all;
pub mod condition;

pub mod abi;

pub mod add;
pub mod binop;
pub mod context;
pub mod div;
pub mod far_call;
pub mod invalid_opcode;
pub mod jump;
pub mod log;
pub mod mul;
pub mod near_call;
pub mod noop;
pub mod ptr;
pub mod ret;
pub mod shift;
pub mod sub;
pub mod uma;

pub mod versioned_hash;

pub use self::add::*;
pub use self::binop::*;
pub use self::context::*;
pub use self::div::*;
pub use self::far_call::*;
pub use self::invalid_opcode::*;
pub use self::jump::*;
pub use self::log::*;
pub use self::mul::*;
pub use self::near_call::*;
pub use self::noop::*;
pub use self::ptr::*;
pub use self::ret::*;
pub use self::shift::*;
pub use self::sub::*;
pub use self::uma::*;

pub use self::abi::*;
pub use self::all::*;
pub use self::condition::*;

pub use self::versioned_hash::*;

pub const NUM_OPCODES: usize = 16;

pub const INVALID_OPCODE_VARIANT: OpcodeVariant = OpcodeVariant {
    opcode: Opcode::Invalid(InvalidOpcode),
    src0_operand_type: Operand::RegOnly,
    dst0_operand_type: Operand::RegOnly,
    flags: [false; NUM_NON_EXCLUSIVE_FLAGS],
};

pub(crate) fn all_opcodes() -> Vec<Box<dyn OpcodeProps + 'static>> {
    let opcodes = vec![
        Box::new(InvalidOpcode) as Box<dyn OpcodeProps>,
        Box::new(NopOpcode),
        Box::new(AddOpcode::Add),
        Box::new(SubOpcode::Sub),
        Box::new(MulOpcode),
        Box::new(DivOpcode),
        Box::new(JumpOpcode),
        Box::new(BinopOpcode::Xor),
        Box::new(ShiftOpcode::Shl),
        Box::new(PtrOpcode::Add),
        Box::new(NearCallOpcode),
        Box::new(ContextOpcode::This),
        Box::new(LogOpcode::StorageRead),
        Box::new(FarCallOpcode::Normal),
        Box::new(RetOpcode::Ok),
        Box::new(UMAOpcode::HeapRead),
    ];

    assert_eq!(opcodes.len(), NUM_OPCODES, "number of opcodes mismatch");
    assert_eq!(
        opcodes[0].name(),
        "Invalid opcode",
        "first opcode should be invalid opcode"
    );

    opcodes
}

pub fn all_opcode_prototypes() -> [Opcode; NUM_OPCODES] {
    let opcodes = [
        Opcode::Invalid(InvalidOpcode),
        Opcode::Nop(NopOpcode),
        Opcode::Add(AddOpcode::Add),
        Opcode::Sub(SubOpcode::Sub),
        Opcode::Mul(MulOpcode),
        Opcode::Div(DivOpcode),
        Opcode::Jump(JumpOpcode),
        Opcode::Binop(BinopOpcode::Xor),
        Opcode::Shift(ShiftOpcode::Shl),
        Opcode::Ptr(PtrOpcode::Add),
        Opcode::NearCall(NearCallOpcode),
        Opcode::Context(ContextOpcode::This),
        Opcode::Log(LogOpcode::StorageRead),
        Opcode::FarCall(FarCallOpcode::Normal),
        Opcode::Ret(RetOpcode::Ok),
        Opcode::UMA(UMAOpcode::HeapRead),
    ];

    opcodes
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpcodeVariant {
    pub opcode: Opcode,
    pub src0_operand_type: Operand,
    pub dst0_operand_type: Operand,
    pub flags: [bool; NUM_NON_EXCLUSIVE_FLAGS],
}

pub const SET_FLAGS_FLAG_IDX: usize = 0;
pub const SWAP_OPERANDS_FLAG_IDX_FOR_ARITH_OPCODES: usize = 1;
pub const SWAP_OPERANDS_FLAG_IDX_FOR_PTR_OPCODE: usize = 0;

impl OpcodeVariant {
    pub fn ergs_price(&self) -> u32 {
        self.opcode.ergs_price()
    }

    pub const fn swap_operands(&self) -> bool {
        match self.opcode {
            Opcode::Sub(_) | Opcode::Div(_) | Opcode::Shift(_) => {
                self.flags[SWAP_OPERANDS_FLAG_IDX_FOR_ARITH_OPCODES]
            }
            Opcode::Ptr(_) => self.flags[SWAP_OPERANDS_FLAG_IDX_FOR_PTR_OPCODE],
            _ => false,
        }
    }

    pub fn requires_kernel_mode(&self) -> bool {
        self.opcode.requires_kernel_mode()
    }

    pub fn can_be_used_in_static_context(&self) -> bool {
        self.opcode.can_be_used_in_static_context()
    }

    // At the moment "invalid" opcode burns all the ergs,
    // but then nevertheless is masked into "ret.panic r0".
    // This is an implementation detail and may change in the future
    pub fn is_explicit_panic(&self) -> bool {
        match &self.opcode {
            Opcode::Invalid(_) => true,
            _ => false,
        }
    }
}

impl std::default::Default for OpcodeVariant {
    fn default() -> Self {
        OpcodeVariant {
            opcode: Opcode::Nop(NopOpcode),
            src0_operand_type: Operand::RegOnly,
            dst0_operand_type: Operand::RegOnly,
            flags: [false; NUM_NON_EXCLUSIVE_FLAGS],
        }
    }
}

impl std::fmt::Display for OpcodeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Opcode and variant: {:?}", self.opcode)?;
        writeln!(f, "Src0 operand type: {:?}", self.src0_operand_type)?;
        writeln!(f, "Dst0 operand type: {:?}", self.dst0_operand_type)?;
        write!(
            f,
            "Non-exclusive flags: {:#01}|{:#01}",
            self.flags[0], self.flags[1],
        )
    }
}

pub fn synthesize_opcode_decoding_tables(
    table_bits: usize,
    up_to_version: ISAVersion,
) -> Vec<OpcodeVariant> {
    // we walk over all opcode variants, and perform passes:
    // - first just encode everything and pad the table with invalid opcodes,
    // with an extra convension that all zeroes is invalid opcode
    // - replace invalid opcode with Ret::Panic

    let all_opcodes = all_opcodes();
    let mut checker = std::collections::HashSet::new();
    for (i, opcode) in all_opcodes.iter().enumerate() {
        let is_new = checker.insert(opcode.name());
        if !is_new {
            panic!(
                "Duplicate opcode with name: {} at index {}",
                opcode.name(),
                i
            );
        }
    }

    let mut result = Vec::with_capacity(1 << table_bits);

    // we walk over the versions to get only opcodes added in the correspoding one

    for version in ALL_ISA_VERSIONS.iter() {
        if version > &up_to_version {
            break;
        }

        let all_opcodes_in_version = all_variants_in_version(*version);
        let new_opcodes = find_new_opcodes(&mut result, &all_opcodes_in_version);

        result.extend(new_opcodes);

        assert_eq!(result.len(), compute_encoding_density(*version));
    }

    assert_eq!(result[0], INVALID_OPCODE_VARIANT);
    assert!(result.len() <= 1 << table_bits);
    result.resize(1 << table_bits, INVALID_OPCODE_VARIANT);

    result
}

pub fn synthesize_opcode_decoding_tables_legacy(
    table_bits: usize,
    up_to_version: ISAVersion,
) -> Vec<OpcodeVariant> {
    // we walk over all opcode variants, and perform passes:
    // - first just encode everything and pad the table with invalid opcodes,
    // with an extra convension that all zeroes is invalid opcode
    // - replace invalid opcode with Ret::Panic

    let all_opcodes = all_opcodes();
    let all_prototypes = all_opcode_prototypes();

    let mut checker = std::collections::HashSet::new();
    for (i, opcode) in all_opcodes.iter().enumerate() {
        let is_new = checker.insert(opcode.name());
        if !is_new {
            panic!(
                "Duplicate opcode with name: {} at index {}",
                opcode.name(),
                i
            );
        }
    }

    let mut result = Vec::with_capacity(1 << table_bits);

    // we walk over the versions to get only opcodes added in the correspoding one

    for version in ALL_ISA_VERSIONS.iter() {
        if version > &up_to_version {
            break;
        }

        // walk over prototypes and use their declared information

        for (i, opcode) in all_opcodes.iter().enumerate() {
            if i == 0 {
                assert_eq!(
                    opcode.name(),
                    "Invalid opcode",
                    "first opcode should be invalid opcode"
                );
            }
            let all_variants_added_in_this_version = opcode.variants_data(*version);
            match (
                &opcode.input_operands(*version)[..],
                &opcode.output_operands(*version)[..],
            ) {
                (&[], &[])
                | (&[Operand::RegOnly], &[])
                | (&[], &[Operand::RegOnly])
                | (&[Operand::RegOnly], &[Operand::RegOnly])
                | (&[Operand::RegOnly, Operand::RegOnly], &[])
                | (&[Operand::RegOnly, Operand::RegOnly], &[Operand::RegOnly])
                | (&[Operand::RegOnly], &[Operand::RegOnly, Operand::RegOnly]) => {
                    // we do not need to encode any extra information, just a variant
                    for variant_data in all_variants_added_in_this_version.into_iter() {
                        let variant_idx = variant_data.variant_idx;
                        let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                        let mut tmp = INVALID_OPCODE_VARIANT;
                        tmp.opcode = all_prototypes[i]
                            .materialize_subvariant_from_prototype(variant_idx, version);
                        match num_non_exclusive_flags {
                            0 => {
                                // only 1 option
                                result.push(tmp);
                            }
                            1 => {
                                // 2 options
                                for b in [false, true] {
                                    let mut t = tmp;
                                    t.flags[0] = b;
                                    result.push(t);
                                }
                            }
                            2 => {
                                // 4 options
                                for b0 in [false, true] {
                                    for b1 in [false, true] {
                                        let mut t = tmp;
                                        t.flags[0] = b0;
                                        t.flags[1] = b1;
                                        result.push(t);
                                    }
                                }
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                }
                (&[Operand::Full(_)], &[Operand::Full(_)])
                | (&[Operand::Full(_), Operand::RegOnly], &[Operand::Full(_)])
                | (&[Operand::Full(_), Operand::RegOnly], &[Operand::Full(_), Operand::RegOnly]) => {
                    // we need to encode all combinations of input and output being reg/stack/code/imm
                    for variant_data in all_variants_added_in_this_version.into_iter() {
                        let variant_idx = variant_data.variant_idx;
                        let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                        for input_variant in ImmMemHandlerFlags::all_variants() {
                            for output_variant in ImmMemHandlerFlags::all_variants() {
                                let mut tmp = INVALID_OPCODE_VARIANT;
                                tmp.opcode = all_prototypes[i]
                                    .materialize_subvariant_from_prototype(variant_idx, version);
                                tmp.src0_operand_type = Operand::Full(input_variant);
                                if !output_variant.is_allowed_for_dst() {
                                    // some of the variants are not applicable for dst
                                    continue;
                                }
                                tmp.dst0_operand_type = Operand::Full(output_variant);
                                match num_non_exclusive_flags {
                                    0 => {
                                        result.push(tmp);
                                    }
                                    1 => {
                                        for b in [false, true] {
                                            let mut t = tmp;
                                            t.flags[0] = b;
                                            result.push(t);
                                        }
                                    }
                                    2 => {
                                        for b0 in [false, true] {
                                            for b1 in [false, true] {
                                                let mut t = tmp;
                                                t.flags[0] = b0;
                                                t.flags[1] = b1;
                                                result.push(t);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!()
                                    }
                                }
                            }
                        }
                    }
                }
                (&[Operand::Full(_)], &[]) => {
                    // we need to encode all combinations of input and output being reg/stack/code/imm
                    // but we only have a case of inputs
                    for variant_data in all_variants_added_in_this_version.into_iter() {
                        let variant_idx = variant_data.variant_idx;
                        let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                        for input_variant in ImmMemHandlerFlags::all_variants() {
                            let mut tmp = INVALID_OPCODE_VARIANT;
                            tmp.opcode = all_prototypes[i]
                                .materialize_subvariant_from_prototype(variant_idx, version);
                            tmp.src0_operand_type = Operand::Full(input_variant);
                            match num_non_exclusive_flags {
                                0 => {
                                    result.push(tmp);
                                }
                                1 => {
                                    for b in [false, true] {
                                        let mut t = tmp;
                                        t.flags[0] = b;
                                        result.push(t);
                                    }
                                }
                                2 => {
                                    for b0 in [false, true] {
                                        for b1 in [false, true] {
                                            let mut t = tmp;
                                            t.flags[0] = b0;
                                            t.flags[1] = b1;
                                            result.push(t);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                    }
                }
                a @ _ => {
                    unreachable!(
                        "unknown combination of operand types: {:?} for opcode {}",
                        a,
                        opcode.name()
                    );
                }
            }
        }

        assert_eq!(result.len(), compute_encoding_density(*version));
    }

    assert_eq!(result[0], INVALID_OPCODE_VARIANT);
    assert!(result.len() <= 1 << table_bits);
    result.resize(1 << table_bits, INVALID_OPCODE_VARIANT);

    result
}

fn all_variants_in_version(version: ISAVersion) -> Vec<OpcodeVariant> {
    // we walk over all opcode variants, and perform passes:
    // - first just encode everything and pad the table with invalid opcodes,
    // with an extra convension that all zeroes is invalid opcode
    // - replace invalid opcode with Ret::Panic

    let all_opcodes = all_opcodes();
    let all_prototypes = all_opcode_prototypes();

    let mut checker = std::collections::HashSet::new();
    for (i, opcode) in all_opcodes.iter().enumerate() {
        let is_new = checker.insert(opcode.name());
        if !is_new {
            panic!(
                "Duplicate opcode with name: {} at index {}",
                opcode.name(),
                i
            );
        }
    }

    let mut result = Vec::with_capacity(1 << 11);

    // we walk over the versions to get only opcodes added in the correspoding one

    // walk over prototypes and use their declared information

    for (i, opcode) in all_opcodes.iter().enumerate() {
        if i == 0 {
            assert_eq!(
                opcode.name(),
                "Invalid opcode",
                "first opcode should be invalid opcode"
            );
        }
        let all_variants_in_this_version = opcode.variants_data(version);

        // cycle over all variants without addressing mode yet
        for variant_data in all_variants_in_this_version.into_iter() {
            // each variant may have different addressing modes, specific for itself
            let variant_idx = variant_data.variant_idx;
            let concrete_opcode_without_addressing =
                all_prototypes[i].materialize_subvariant_from_prototype(variant_idx, &version);

            // now match over addressing modes
            match (
                &concrete_opcode_without_addressing.input_operands(version)[..],
                &concrete_opcode_without_addressing.output_operands(version)[..],
            ) {
                (&[], &[])
                | (&[Operand::RegOnly], &[])
                | (&[], &[Operand::RegOnly])
                | (&[Operand::RegOnly], &[Operand::RegOnly])
                | (&[Operand::RegOnly, Operand::RegOnly], &[])
                | (&[Operand::RegOnly, Operand::RegOnly], &[Operand::RegOnly])
                | (&[Operand::RegOnly], &[Operand::RegOnly, Operand::RegOnly]) => {
                    // we do not need to encode any extra information, just a variant
                    let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                    let mut tmp = INVALID_OPCODE_VARIANT;
                    tmp.opcode = all_prototypes[i]
                        .materialize_subvariant_from_prototype(variant_idx, &version);
                    assert!(matches!(tmp.src0_operand_type, Operand::RegOnly));
                    assert!(matches!(tmp.dst0_operand_type, Operand::RegOnly));
                    match num_non_exclusive_flags {
                        0 => {
                            // only 1 option
                            result.push(tmp);
                        }
                        1 => {
                            // 2 options
                            for b in [false, true] {
                                let mut t = tmp;
                                t.flags[0] = b;
                                result.push(t);
                            }
                        }
                        2 => {
                            // 4 options
                            for b0 in [false, true] {
                                for b1 in [false, true] {
                                    let mut t = tmp;
                                    t.flags[0] = b0;
                                    t.flags[1] = b1;
                                    result.push(t);
                                }
                            }
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                (&[Operand::Full(_)], &[Operand::Full(_)])
                | (&[Operand::Full(_), Operand::RegOnly], &[Operand::Full(_)])
                | (&[Operand::Full(_), Operand::RegOnly], &[Operand::Full(_), Operand::RegOnly]) => {
                    // we need to encode all combinations of input and output being reg/stack/code/imm
                    let variant_idx = variant_data.variant_idx;
                    let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                    for input_variant in ImmMemHandlerFlags::all_variants() {
                        for output_variant in ImmMemHandlerFlags::all_variants() {
                            let mut tmp = INVALID_OPCODE_VARIANT;
                            tmp.opcode = all_prototypes[i]
                                .materialize_subvariant_from_prototype(variant_idx, &version);
                            tmp.src0_operand_type = Operand::Full(input_variant);
                            if !output_variant.is_allowed_for_dst() {
                                continue;
                            }
                            tmp.dst0_operand_type = Operand::Full(output_variant);
                            match num_non_exclusive_flags {
                                0 => {
                                    result.push(tmp);
                                }
                                1 => {
                                    for b in [false, true] {
                                        let mut t = tmp;
                                        t.flags[0] = b;
                                        result.push(t);
                                    }
                                }
                                2 => {
                                    for b0 in [false, true] {
                                        for b1 in [false, true] {
                                            let mut t = tmp;
                                            t.flags[0] = b0;
                                            t.flags[1] = b1;
                                            result.push(t);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                    }
                }
                (&[Operand::Full(_)], &[]) => {
                    // we need to encode all combinations of input and output being reg/stack/code/imm
                    // but we only have a case of inputs
                    let variant_idx = variant_data.variant_idx;
                    let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                    for input_variant in ImmMemHandlerFlags::all_variants() {
                        let mut tmp = INVALID_OPCODE_VARIANT;
                        tmp.opcode = all_prototypes[i]
                            .materialize_subvariant_from_prototype(variant_idx, &version);
                        tmp.src0_operand_type = Operand::Full(input_variant);
                        assert!(matches!(tmp.dst0_operand_type, Operand::RegOnly));
                        match num_non_exclusive_flags {
                            0 => {
                                result.push(tmp);
                            }
                            1 => {
                                for b in [false, true] {
                                    let mut t = tmp;
                                    t.flags[0] = b;
                                    result.push(t);
                                }
                            }
                            2 => {
                                for b0 in [false, true] {
                                    for b1 in [false, true] {
                                        let mut t = tmp;
                                        t.flags[0] = b0;
                                        t.flags[1] = b1;
                                        result.push(t);
                                    }
                                }
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                }
                (&[Operand::RegOrImm(_), Operand::RegOnly], &[Operand::RegOnly])
                | (&[Operand::RegOrImm(_)], &[Operand::RegOnly, Operand::RegOnly]) => {
                    // we need to encode all combinations of input and output being reg/stack/code/imm
                    let variant_idx = variant_data.variant_idx;
                    let num_non_exclusive_flags = variant_data.num_non_exclusive_flags;
                    for src_variant in RegOrImmFlags::all_variants() {
                        let mut tmp = INVALID_OPCODE_VARIANT;
                        tmp.opcode = all_prototypes[i]
                            .materialize_subvariant_from_prototype(variant_idx, &version);
                        tmp.src0_operand_type = Operand::RegOrImm(src_variant);
                        assert!(matches!(tmp.dst0_operand_type, Operand::RegOnly));
                        match num_non_exclusive_flags {
                            0 => {
                                result.push(tmp);
                            }
                            1 => {
                                for b in [false, true] {
                                    let mut t = tmp;
                                    t.flags[0] = b;
                                    result.push(t);
                                }
                            }
                            2 => {
                                for b0 in [false, true] {
                                    for b1 in [false, true] {
                                        let mut t = tmp;
                                        t.flags[0] = b0;
                                        t.flags[1] = b1;
                                        result.push(t);
                                    }
                                }
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                }
                a @ _ => {
                    unreachable!(
                        "unknown combination of operand types: {:?} for opcode {}",
                        a,
                        opcode.name()
                    );
                }
            }
        }
    }

    assert_eq!(result[0], INVALID_OPCODE_VARIANT);

    result
}

// computes including backward compatibility
pub fn max_num_variants_for_version(version: ISAVersion) -> usize {
    let all_opcodes = all_opcodes();
    let max_num_variants = all_opcodes
        .iter()
        .map(|op| op.max_variant_idx(version) + 1) // it's 0 enumerated, so +1
        .max()
        .unwrap();

    max_num_variants
}

// computes including backward compatibility
pub fn max_num_flags_for_version(version: ISAVersion) -> usize {
    let all_opcodes = all_opcodes();
    let num_flags = all_opcodes
        .iter()
        .map(|op| {
            ALL_ISA_VERSIONS
                .iter()
                .filter(|el| *el <= &version)
                .map(|el| {
                    let max_flags_for_added_in_this_version = op
                        .variants_data(*el)
                        .into_iter()
                        .map(|t| t.num_non_exclusive_flags)
                        .max()
                        .unwrap_or(0);

                    max_flags_for_added_in_this_version
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    num_flags
}

pub fn synthesize_bit_decomposition_table(
    opcodes: &[OpcodeVariant],
    assert_version: ISAVersion,
) -> Vec<u64> {
    let all_opcodes = all_opcodes();
    let num_opcodes = all_opcodes.len();
    let input_flags = ImmMemHandlerFlags::num_src_variants();
    let output_flags = ImmMemHandlerFlags::num_dst_variants();
    assert_eq!(num_opcodes, OPCODE_TYPE_BITS);
    assert_eq!(input_flags, OPCODE_INPUT_VARIANT_FLAGS);
    assert_eq!(output_flags, OPCODE_OUTPUT_VARIANT_FLAGS);

    let mut result = Vec::with_capacity(opcodes.len());
    for opcode in opcodes.iter() {
        let encoding = opcode_as_integer_bitmask(opcode, assert_version);
        result.push(encoding);
    }

    result
}

// we only need opcodes added in this particular version, not the cumulative set
// from all the versions below
pub(crate) fn compute_encoding_density(for_version: ISAVersion) -> usize {
    let all_opcodes = all_opcodes();

    let mut checker = std::collections::HashSet::new();
    for (i, opcode) in all_opcodes.iter().enumerate() {
        let is_new = checker.insert(opcode.name());
        if !is_new {
            panic!(
                "Duplicate opcode with name: {} at index {}",
                opcode.name(),
                i
            );
        }
    }

    let mut num_options = 0;
    let all_prototypes = all_opcode_prototypes();
    for (idx, opcode) in all_opcodes.iter().enumerate() {
        let variants_data = opcode.variants_data(for_version);
        for variant in variants_data.into_iter() {
            let variant_idx = variant.variant_idx;
            let concrete_opcode_without_addressing = all_prototypes[idx]
                .materialize_subvariant_from_prototype(variant_idx, &for_version);
            let mut num_addressing_modes = 1;
            for el in concrete_opcode_without_addressing.input_operands(for_version) {
                let multiplier = match el {
                    Operand::Full(_) => ImmMemHandlerFlags::num_src_variants(),
                    Operand::RegOrImm(_) => RegOrImmFlags::num_src_variants(),
                    Operand::RegOnly => 1,
                };
                num_addressing_modes *= multiplier;
            }

            for el in concrete_opcode_without_addressing.output_operands(for_version) {
                let multiplier = match el {
                    Operand::Full(_) => ImmMemHandlerFlags::num_dst_variants(),
                    Operand::RegOrImm(_) => RegOrImmFlags::num_dst_variants(),
                    Operand::RegOnly => 1,
                };
                num_addressing_modes *= multiplier;
            }
            let multiplier_for_non_exclusive = 1 << variant.num_non_exclusive_flags;
            let all_variants = multiplier_for_non_exclusive * num_addressing_modes;
            num_options += all_variants;
        }
    }

    num_options
}

pub(crate) fn opcode_as_integer_bitmask(
    full_opcode_variant: &OpcodeVariant,
    version: ISAVersion,
) -> u64 {
    let max_variant_bits_in_version = max_num_variants_for_version(version);
    let max_num_flags_in_version = max_num_flags_for_version(version);

    let mut encoding = 0u64;
    let mut global_shift = 0;
    // first encoding logical opcode
    encoding |= 1u64 << (global_shift + full_opcode_variant.opcode.variant_idx());
    global_shift += *crate::NUM_LOGICAL_OPCODES;
    // then encode sub-variant
    encoding |= 1u64 << (global_shift + full_opcode_variant.opcode.materialize_subvariant_idx());
    global_shift += max_variant_bits_in_version;
    // then encode flags
    for (i, flag) in full_opcode_variant.flags.iter().enumerate() {
        if *flag {
            encoding |= 1u64 << (global_shift + i);
        }
    }
    global_shift += max_num_flags_in_version;
    // then encode src0 addressing mode
    encoding |= 1u64 << (global_shift + full_opcode_variant.src0_operand_type.variant_idx());
    global_shift += *crate::NUM_INPUT_VARIANTS;
    // then dst0
    encoding |= 1u64 << (global_shift + full_opcode_variant.dst0_operand_type.variant_idx());
    global_shift += *crate::NUM_OUTPUT_VARIANTS;

    let aligned_num_description_bits = total_description_bits_rounded_for_version(version);
    assert!(global_shift <= aligned_num_description_bits);
    global_shift = aligned_num_description_bits;
    // then any auxilary information
    // for now only kernel mode requirement
    if full_opcode_variant.requires_kernel_mode() {
        encoding |= 1u64 << global_shift;
    }
    global_shift += crate::KERNEL_MODE_FLAG_BITS;
    if full_opcode_variant.can_be_used_in_static_context() {
        encoding |= 1u64 << global_shift;
    }
    global_shift += crate::CAN_BE_USED_IN_STATIC_CONTEXT_FLAG_BITS;
    // we also encode any INVALID opcode as explicit panic
    if full_opcode_variant.is_explicit_panic() {
        encoding |= 1u64 << global_shift;
    }
    global_shift += crate::EXPLICIT_PANIC_FLAG_BITS;

    assert!(global_shift <= 64);

    encoding
}

fn semantically_equal(a: &OpcodeVariant, b: &OpcodeVariant) -> bool {
    if a.opcode != b.opcode {
        return false;
    }
    if a.flags != b.flags {
        return false;
    }
    // first compare DST as it can not have difference between Reg only and RegImm
    if a.dst0_operand_type != b.dst0_operand_type {
        return false;
    }

    if a.src0_operand_type == b.src0_operand_type {
        return true;
    }

    match (a.src0_operand_type, b.src0_operand_type) {
        (Operand::RegOnly, Operand::RegOrImm(RegOrImmFlags::UseRegOnly))
        | (Operand::RegOrImm(RegOrImmFlags::UseRegOnly), Operand::RegOnly) => true,
        _ => false,
    }
}

fn find_new_opcodes(old: &mut [OpcodeVariant], new: &[OpcodeVariant]) -> Vec<OpcodeVariant> {
    // sanity check that we always expand
    let tmp: HashSet<OpcodeVariant> = HashSet::from_iter(new.iter().copied());
    for el in old.iter() {
        if tmp.contains(&el) == false {
            // we may have semantic equivalents instead
            let mut found_equal = false;
            for new in new.iter() {
                if found_equal {
                    break;
                }
                if semantically_equal(new, el) {
                    found_equal = true;
                }
            }

            assert!(found_equal, "new set doesn't contain {:?}", el);
        }
    }

    let mut difference = vec![];

    for new in new.iter() {
        let mut found_equivalent = false;
        for old in old.iter_mut() {
            if found_equivalent {
                break;
            }

            if semantically_equal(new, old) {
                found_equivalent = true;
                // also replace them
                *old = *new;
            }
        }

        if found_equivalent == false {
            difference.push(*new);
        }
    }

    difference
}

pub fn compute_decoding_format(assert_version: ISAVersion) -> String {
    let all_opcodes = all_opcodes();
    let num_opcodes = all_opcodes.len();
    let num_options = max_num_variants_for_version(assert_version);
    let num_flags = max_num_flags_for_version(assert_version);
    let input_flags = ImmMemHandlerFlags::num_src_variants();
    let output_flags = ImmMemHandlerFlags::num_dst_variants();

    let total_bits = num_opcodes + num_options + num_flags + input_flags + output_flags;

    format!("{} bits in total.\n{} bit flags for opcode|{} bit flags for variant|{} bit flags for flags|{} bits flags for input type|{} bit flags for output type",
        total_bits,
        num_opcodes,
        num_options,
        num_flags,
        input_flags,
        output_flags
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn show_num_variants() {
        let all = compute_encoding_density(ISAVersion(0));
        println!(
            "Need to encode {} variants in version {:?}",
            all,
            ISAVersion(0)
        );
        let all = compute_encoding_density(ISAVersion(1));
        println!(
            "Need to encode {} variants in version {:?}",
            all,
            ISAVersion(1)
        );
    }

    #[test]
    fn show_circuit_decoding_format() {
        let format = compute_decoding_format(ISAVersion(0));
        println!("{} for version {:?}", format, ISAVersion(0));
        let format = compute_decoding_format(ISAVersion(1));
        println!("{} for version {:?}", format, ISAVersion(1));
    }

    #[test]
    fn synthesize_table() {
        let table = synthesize_opcode_decoding_tables(11, ISAVersion(0));
        for el in table[0..20].iter() {
            println!("Opcode {:?}", el);
        }
    }

    #[test]
    fn synthesize_bit_encoding_table() {
        let table = synthesize_opcode_decoding_tables(11, ISAVersion(0));
        let encoding = synthesize_bit_decomposition_table(&table, ISAVersion(0));
        for (el, enc) in table.iter().zip(encoding.iter()).skip(40).take(20) {
            println!("Opcode: {:?}\n Encoding {:035b}", el, enc);
        }
    }

    #[test]
    fn check_encoding() {
        let variant = OpcodeVariant {
            opcode: Opcode::Add(AddOpcode::Add),
            src0_operand_type: Operand::Full(ImmMemHandlerFlags::UseImm16Only),
            dst0_operand_type: Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            flags: [false, false],
        };
        let index = OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP.get(&variant).unwrap();
        dbg!(index);
        let encoding = OPCODES_PROPS_INTEGER_BITMASKS[*index];
        println!("Opcode: {:?}\n Encoding {:048b}", variant, encoding);
    }

    #[test]
    fn check_serialize() {
        let variant = OpcodeVariant {
            opcode: Opcode::Log(LogOpcode::PrecompileCall),
            src0_operand_type: Operand::RegOnly,
            dst0_operand_type: Operand::RegOnly,
            flags: [false, false],
        };
        let index = OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP.get(&variant).unwrap();
        dbg!(index);
        let encoding = OPCODES_PROPS_INTEGER_BITMASKS[*index];
        println!("Opcode: {:?}\n Encoding {:048b}", variant, encoding);
    }

    fn decode_from_raw(raw: u64) -> OpcodeVariant {
        let properties_index = raw & ((1u64 << OPCODES_TABLE_WIDTH) - 1);
        let opcode = OPCODES_TABLE[properties_index as usize];

        opcode
    }

    #[test]
    fn decode_opcode() {
        let opcode = 0x000300000004001d;

        // Previous code word 0 = 0x0000000300010369
        // Previous code word 1 = 0x000000020000036d
        // Previous code word 2 = 0x0000000000000368
        // Previous code word 3 = 0x000000010100003a
        let variant = decode_from_raw(opcode);
        dbg!(variant);
    }

    #[test]
    fn calculator() {
        let mut v0 = all_variants_in_version(ISAVersion(0));
        dbg!(v0.len());
        let v1 = all_variants_in_version(ISAVersion(1));
        dbg!(v1.len());

        // check that our ordering doesn't change for legacy tables
        let legacy_v0 = synthesize_opcode_decoding_tables_legacy(11, ISAVersion(0));
        assert_eq!(&legacy_v0[..v0.len()], &v0[..]);

        let new_v0 = synthesize_opcode_decoding_tables(11, ISAVersion(0));
        let num_non_trivial_v0 = compute_encoding_density(ISAVersion(0));
        assert_eq!(&legacy_v0[..], &new_v0[..]);

        let difference = find_new_opcodes(&mut v0, &v1);

        let new_v1 = synthesize_opcode_decoding_tables(11, ISAVersion(1));
        let num_non_trivial_v1 = compute_encoding_density(ISAVersion(1));

        for (a, b) in new_v0[..num_non_trivial_v0]
            .iter()
            .zip(new_v1[..num_non_trivial_v0].iter())
        {
            assert!(semantically_equal(a, b));
        }

        assert!(num_non_trivial_v1 >= num_non_trivial_v0);

        for idx in num_non_trivial_v0..num_non_trivial_v1 {
            assert!(new_v0[idx].is_explicit_panic());
        }

        dbg!(&difference);
    }
}
