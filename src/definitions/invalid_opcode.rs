use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidOpcode;

impl OpcodeVariantProps for InvalidOpcode {
    fn all_variants() -> Vec<Self> {
        vec![InvalidOpcode]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        InvalidOpcode.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        0
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            0 => Some(InvalidOpcode),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        INVALID_OPCODE_ERGS
    }
}

impl OpcodeProps for InvalidOpcode {
    fn name(&self) -> &'static str {
        "Invalid opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 0, 0)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 0, 0)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        InvalidOpcode.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![]
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        vec![]
    }
    fn requires_kernel_mode(&self) -> bool {
        false
    }
    fn can_be_used_in_static_context(&self) -> bool {
        true
    }
}
