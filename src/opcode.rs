use super::*;

use crate::decoding::VmEncodingMode;
use crate::{Condition, ImmMemHandlerFlags, NopOpcode, OpcodeVariant, Operand, RetOpcode};

use crate::decoding::encoding_mode_production::EncodingModeProduction;
use crate::definitions::all::Opcode;

#[derive(Clone, Copy, Debug)]
pub struct DecodedOpcode<const N: usize = 8, E: VmEncodingMode<N> = EncodingModeProduction> {
    pub variant: OpcodeVariant,
    pub condition: Condition,
    pub src0_reg_idx: u8,
    pub src1_reg_idx: u8,
    pub dst0_reg_idx: u8,
    pub dst1_reg_idx: u8,
    pub imm_0: E::PcOrImm,
    pub imm_1: E::PcOrImm,
}

impl<const N: usize, E: VmEncodingMode<N>> Default for DecodedOpcode<N, E> {
    fn default() -> Self {
        DecodedOpcode {
            variant: OpcodeVariant::default(),
            condition: Condition::Always,
            src0_reg_idx: 0,
            src1_reg_idx: 0,
            dst0_reg_idx: 0,
            dst1_reg_idx: 0,
            imm_0: Default::default(),
            imm_1: Default::default(),
        }
    }
}

impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
    pub fn mask_into_panic(&mut self) {
        // we only mask first 8 bytes of the opcode, and do not touch regs or imms
        self.variant.opcode = Opcode::Ret(RetOpcode::Panic);
        // it's important that we set condition to "always"
        self.condition = Condition::Always;
        // the rest is just trivial downgrade
        self.variant.src0_operand_type = Operand::RegOnly;
        self.variant.dst0_operand_type = Operand::RegOnly;
        // ret opcode doesn't use flags, so it's always empty
        self.variant.flags = [false; NUM_NON_EXCLUSIVE_FLAGS];
        self.src0_reg_idx = 0;
        self.src1_reg_idx = 0;
        self.dst0_reg_idx = 0;
        self.dst1_reg_idx = 0;
    }

    pub fn mask_into_nop(&mut self) {
        self.variant.opcode = Opcode::Nop(NopOpcode);
        // it's important that we set condition to "always"
        self.condition = Condition::Always;
        // the rest is just trivial downgrade
        self.variant.src0_operand_type = Operand::Full(ImmMemHandlerFlags::UseRegOnly);
        self.variant.dst0_operand_type = Operand::Full(ImmMemHandlerFlags::UseRegOnly);
        self.variant.flags = [false; NUM_NON_EXCLUSIVE_FLAGS];
        self.src0_reg_idx = 0;
        self.src1_reg_idx = 0;
        self.dst0_reg_idx = 0;
        self.dst1_reg_idx = 0;
    }

    pub fn serialize_as_integer(&self) -> E::IntegerRepresentation {
        E::encode_as_integer(&self)
    }

    pub fn serialize_as_bytes(&self) -> [u8; N] {
        E::encode_as_bytes(&self)
    }
}

impl<const N: usize, E: VmEncodingMode<N>> std::fmt::Display for DecodedOpcode<N, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "Decoded opcode")?;
        writeln!(f, "{}", self.variant)?;
        writeln!(f, "Condition: {:?}", self.condition)?;
        writeln!(
            f,
            "Register selections: src0 = r{}, src1 = r{}, dst0 = r{}, dst1 = r{}",
            self.src0_reg_idx, self.src1_reg_idx, self.dst0_reg_idx, self.dst1_reg_idx
        )?;
        writeln!(f, "IMM0 constant: {:#08x}", self.imm_0)?;
        write!(f, "IMM1 constant: {:#08x}", self.imm_1)
    }
}
