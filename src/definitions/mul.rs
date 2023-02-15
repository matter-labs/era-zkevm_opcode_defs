use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MulOpcode;

impl OpcodeVariantProps for MulOpcode {
    fn all_variants() -> Vec<Self> {
        vec![MulOpcode]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        MulOpcode.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        0
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            0 => Some(MulOpcode),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        RICH_ADDRESSING_OPCODE_ERGS
    }
}

impl OpcodeProps for MulOpcode {
    fn name(&self) -> &'static str {
        "Mul opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 2)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 2)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        MulOpcode.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![
            Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            Operand::RegOnly,
        ]
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![
            Operand::Full(ImmMemHandlerFlags::UseRegOnly),
            Operand::RegOnly,
        ]
    }
    fn requires_kernel_mode(&self) -> bool {
        false
    }
    fn can_be_used_in_static_context(&self) -> bool {
        true
    }
}
