use super::*;
use ethereum_types::U256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PtrOpcode {
    Add = 0,
    Sub,
    Pack,
    Shrink,
}

pub const MAX_OFFSET_FOR_ADD_SUB: U256 = U256([1u64 << 32, 0, 0, 0]);

impl OpcodeVariantProps for PtrOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            PtrOpcode::Add,
            PtrOpcode::Sub,
            PtrOpcode::Pack,
            PtrOpcode::Shrink,
        ]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        PtrOpcode::Shrink.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == PtrOpcode::Add.variant_index() => Some(PtrOpcode::Add),
            i if i == PtrOpcode::Sub.variant_index() => Some(PtrOpcode::Sub),
            i if i == PtrOpcode::Pack.variant_index() => Some(PtrOpcode::Pack),
            i if i == PtrOpcode::Shrink.variant_index() => Some(PtrOpcode::Shrink),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        RICH_ADDRESSING_OPCODE_ERGS
    }
}

impl OpcodeProps for PtrOpcode {
    fn name(&self) -> &'static str {
        "Ptr opcode"
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
        PtrOpcode::Pack.variant_index()
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
