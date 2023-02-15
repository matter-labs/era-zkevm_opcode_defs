use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ShiftOpcode {
    Shl = 0,
    Shr,
    Rol,
    Ror,
}

impl OpcodeVariantProps for ShiftOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            ShiftOpcode::Shl,
            ShiftOpcode::Shr,
            ShiftOpcode::Rol,
            ShiftOpcode::Ror,
        ]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        ShiftOpcode::Ror.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == ShiftOpcode::Shl.variant_index() => Some(ShiftOpcode::Shl),
            i if i == ShiftOpcode::Shr.variant_index() => Some(ShiftOpcode::Shr),
            i if i == ShiftOpcode::Rol.variant_index() => Some(ShiftOpcode::Rol),
            i if i == ShiftOpcode::Ror.variant_index() => Some(ShiftOpcode::Ror),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        RICH_ADDRESSING_OPCODE_ERGS
    }
}

impl OpcodeProps for ShiftOpcode {
    fn name(&self) -> &'static str {
        "Shift opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 2, 2)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 2, 2)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        ShiftOpcode::Ror.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![
            Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            Operand::RegOnly,
        ]
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![Operand::Full(ImmMemHandlerFlags::UseRegOnly)]
    }
    fn requires_kernel_mode(&self) -> bool {
        false
    }
    fn can_be_used_in_static_context(&self) -> bool {
        true
    }
}
