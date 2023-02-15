use super::*;

use crate::merge_u4;
use crate::split_as_u4;
use crate::Condition;
use crate::OpcodeVariant;
use crate::CONDITIONAL_BITS_SHIFT;
use crate::CONDITIONAL_BITS_WIDTH;
use crate::DST_REGS_SHIFT;
use crate::OPCODES_TABLE;
use crate::OPCODES_TABLE_WIDTH;
use crate::SRC_REGS_SHIFT;
use crate::{NOP_OPCODE_VARIANT, PANIC_OPCODE_VARIANT};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EncodingModeProduction;

impl EncodingModeProduction {
    const IMM0_SHIFT: u32 = 32;
    const IMM1_SHIFT: u32 = 48;
    const VARIANT_AND_CONDITION_BITS: usize = 16;
    const VARIANT_MASK: u64 = (1u64 << OPCODES_TABLE_WIDTH) - 1;
    const CONDITION_MASK: u64 = ((1u64 << CONDITIONAL_BITS_WIDTH) - 1) << CONDITIONAL_BITS_SHIFT;
    const UNUSED_GAP_RAW_MASK: u64 = (1u64
        << (Self::VARIANT_AND_CONDITION_BITS - OPCODES_TABLE_WIDTH - CONDITIONAL_BITS_WIDTH))
        - 1;
    const UNUSED_BITS_MASK: u64 = Self::UNUSED_GAP_RAW_MASK << OPCODES_TABLE_WIDTH;

    pub fn variant_and_condition_from_u64_word(
        raw_value: u64,
    ) -> (OpcodeVariant, Condition, usize) {
        let variant_bits = raw_value & Self::VARIANT_MASK;
        let opcode_variant = OPCODES_TABLE[variant_bits as usize];
        let condition_bits = (raw_value & Self::CONDITION_MASK) >> CONDITIONAL_BITS_SHIFT;
        let condition = Condition::materialize_variant(condition_bits as usize);

        (opcode_variant, condition, variant_bits as usize)
    }
}

impl VmEncodingMode<8> for EncodingModeProduction {
    type PcOrImm = u16;
    type IntegerRepresentation = u64;

    fn nop_encoding() -> Self::IntegerRepresentation {
        let variant = *NOP_OPCODE_VARIANT;
        let full_nop_opcode = DecodedOpcode::<8, Self> {
            variant,
            condition: Condition::Always,
            src0_reg_idx: 0,
            src1_reg_idx: 0,
            dst0_reg_idx: 0,
            dst1_reg_idx: 0,
            imm_0: Self::PcOrImm::from_u64_clipped(0),
            imm_1: Self::PcOrImm::from_u64_clipped(0),
        };

        full_nop_opcode.serialize_as_integer()
    }

    fn exception_revert_encoding() -> Self::IntegerRepresentation {
        // ret.panic r0
        let variant = *PANIC_OPCODE_VARIANT;
        let full_nop_opcode = DecodedOpcode::<8, Self> {
            variant,
            condition: Condition::Always,
            src0_reg_idx: 0,
            src1_reg_idx: 0,
            dst0_reg_idx: 0,
            dst1_reg_idx: 0,
            imm_0: Self::PcOrImm::from_u64_clipped(0),
            imm_1: Self::PcOrImm::from_u64_clipped(0),
        };

        full_nop_opcode.serialize_as_integer()
    }

    fn parse_preliminary_variant_and_absolute_number(
        integer_representaiton: Self::IntegerRepresentation,
    ) -> (DecodedOpcode<8, Self>, VariantMonotonicNumber) {
        let raw_value = integer_representaiton;
        let (variant, condition, variant_bits) =
            Self::variant_and_condition_from_u64_word(raw_value);

        // decode purely as integer
        let src_byte = (raw_value >> SRC_REGS_SHIFT) as u8;
        let dst_byte = (raw_value >> DST_REGS_SHIFT) as u8;

        let imm_0 = (raw_value >> Self::IMM0_SHIFT) as u16;
        let imm_1 = (raw_value >> Self::IMM1_SHIFT) as u16;

        let (src0_reg_idx, src1_reg_idx) = split_as_u4(src_byte);
        let (dst0_reg_idx, dst1_reg_idx) = split_as_u4(dst_byte);

        let new = DecodedOpcode {
            variant,
            condition,
            src0_reg_idx,
            src1_reg_idx,
            dst0_reg_idx,
            dst1_reg_idx,
            imm_0,
            imm_1,
        };

        (new, VariantMonotonicNumber::from_usize(variant_bits))
    }

    fn encode_as_integer(opcode: &DecodedOpcode<8, Self>) -> Self::IntegerRepresentation {
        // we have 11 bits of opcode,
        // then 2 bit gap,
        // then 3 bit conditional,
        // then 4x4 register indexes
        // then 2x8 immediates

        // our encoding is just an integer

        let mut encoding = 0u64;

        let variant_bits = if let Some(variant_bits) = crate::OPCODE_TO_CANONICAL_INDEX_LOOKUP_MAP
            .get(&opcode.variant)
            .copied()
        {
            variant_bits
        } else {
            panic!("Unknown variant {}", opcode.variant);
        };
        assert!(variant_bits.next_power_of_two().trailing_zeros() as usize <= OPCODES_TABLE_WIDTH);

        encoding |= variant_bits as u64;

        // condition
        let condition = opcode.condition.variant_index();
        assert!(condition.next_power_of_two().trailing_zeros() as usize <= CONDITIONAL_BITS_WIDTH);
        encoding |= (condition as u64) << CONDITIONAL_BITS_SHIFT;

        // register indexes
        let src_registers_byte = merge_u4(opcode.src0_reg_idx, opcode.src1_reg_idx);
        encoding |= (src_registers_byte as u64) << SRC_REGS_SHIFT;
        let dst_registers_byte = merge_u4(opcode.dst0_reg_idx, opcode.dst1_reg_idx);
        encoding |= (dst_registers_byte as u64) << DST_REGS_SHIFT;

        encoding |= (opcode.imm_0 as u64) << Self::IMM0_SHIFT;
        encoding |= (opcode.imm_1 as u64) << Self::IMM1_SHIFT;

        encoding
    }

    fn encode_as_bytes(opcode: &DecodedOpcode<8, Self>) -> [u8; 8] {
        Self::encode_as_integer(opcode).encode_as_bytes()
    }

    fn split_pc(pc: Self::PcOrImm) -> (Self::PcOrImm, Self::PcOrImm) {
        (pc >> 2, pc & 0b11)
    }

    fn integer_representaiton_from_u256(
        value: U256,
        index: Self::PcOrImm,
    ) -> Self::IntegerRepresentation {
        match index {
            0 => value.0[3],
            1 => value.0[2],
            2 => value.0[1],
            3 => value.0[0],
            _ => unreachable!(),
        }
    }

    fn is_canonical_encoding(value: Self::IntegerRepresentation) -> bool {
        let unused_bits = value & Self::UNUSED_BITS_MASK;

        unused_bits == 0
    }
}
