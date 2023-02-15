use super::*;

pub const RET_IMPLICIT_RETURNDATA_PARAMS_REGISTER: u8 = 0;
pub const RET_RESERVED_REGISTER_0: u8 = 1;
pub const RET_RESERVED_REGISTER_1: u8 = 2;
pub const RET_RESERVED_REGISTER_2: u8 = 3;

pub const RET_TO_LABEL_BIT_IDX: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RetOpcode {
    Ok = 0,
    Revert,
    Panic,
}

impl OpcodeVariantProps for RetOpcode {
    fn all_variants() -> Vec<Self> {
        vec![RetOpcode::Ok, RetOpcode::Revert, RetOpcode::Panic]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        RetOpcode::Panic.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == RetOpcode::Ok.variant_index() => Some(RetOpcode::Ok),
            i if i == RetOpcode::Revert.variant_index() => Some(RetOpcode::Revert),
            i if i == RetOpcode::Panic.variant_index() => Some(RetOpcode::Panic),
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        AVERAGE_OPCODE_ERGS
    }
}

impl OpcodeProps for RetOpcode {
    fn name(&self) -> &'static str {
        "Ret opcode"
    }
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData> {
        match version {
            ISAVersion(0) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 1)
            }
            ISAVersion(1) => {
                full_variants_product(0..=Self::max_variant_idx_for_version(version), 1, 1)
            }
            _ => unimplemented!(),
        }
    }
    fn max_variant_idx(&self, _version: ISAVersion) -> usize {
        RetOpcode::Panic.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            RetOpcode::Ok | RetOpcode::Revert => vec![Operand::RegOnly],
            RetOpcode::Panic => vec![],
        }
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
