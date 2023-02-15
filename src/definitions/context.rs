use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ContextOpcode {
    This = 0,
    Caller,
    CodeAddress,
    Meta,
    ErgsLeft,
    Sp,
    GetContextU128,
    SetContextU128,
    SetErgsPerPubdataByte,
    IncrementTxNumber,
}

impl OpcodeVariantProps for ContextOpcode {
    fn all_variants() -> Vec<Self> {
        vec![
            ContextOpcode::This,
            ContextOpcode::Caller,
            ContextOpcode::CodeAddress,
            ContextOpcode::Meta,
            ContextOpcode::ErgsLeft,
            ContextOpcode::Sp,
            ContextOpcode::GetContextU128,
            ContextOpcode::SetContextU128,
            ContextOpcode::SetErgsPerPubdataByte,
            ContextOpcode::IncrementTxNumber,
        ]
    }

    fn max_variant_idx_for_version(_version: ISAVersion) -> usize {
        ContextOpcode::IncrementTxNumber.variant_index()
    }

    fn minimal_version(&self) -> ISAVersion {
        ALL_ISA_VERSIONS[0]
    }

    fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }

    fn from_variant_index_for_version(index: usize, _version: &ISAVersion) -> Option<Self> {
        match index {
            i if i == ContextOpcode::This.variant_index() => Some(ContextOpcode::This),
            i if i == ContextOpcode::Caller.variant_index() => Some(ContextOpcode::Caller),
            i if i == ContextOpcode::CodeAddress.variant_index() => {
                Some(ContextOpcode::CodeAddress)
            }
            i if i == ContextOpcode::Meta.variant_index() => Some(ContextOpcode::Meta),
            i if i == ContextOpcode::ErgsLeft.variant_index() => Some(ContextOpcode::ErgsLeft),
            i if i == ContextOpcode::Sp.variant_index() => Some(ContextOpcode::Sp),
            i if i == ContextOpcode::GetContextU128.variant_index() => {
                Some(ContextOpcode::GetContextU128)
            }
            i if i == ContextOpcode::SetContextU128.variant_index() => {
                Some(ContextOpcode::SetContextU128)
            }
            i if i == ContextOpcode::SetErgsPerPubdataByte.variant_index() => {
                Some(ContextOpcode::SetErgsPerPubdataByte)
            }
            i if i == ContextOpcode::IncrementTxNumber.variant_index() => {
                Some(ContextOpcode::IncrementTxNumber)
            }
            _ => None,
        }
    }

    fn ergs_price(&self) -> u32 {
        AVERAGE_OPCODE_ERGS
    }
}

impl OpcodeProps for ContextOpcode {
    fn name(&self) -> &'static str {
        "Context opcode"
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
        ContextOpcode::IncrementTxNumber.variant_index()
    }
    fn input_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            ContextOpcode::SetContextU128 | ContextOpcode::SetErgsPerPubdataByte => {
                vec![Operand::RegOnly]
            }
            _ => vec![],
        }
    }
    fn output_operands(&self, _version: ISAVersion) -> Vec<Operand> {
        match self {
            ContextOpcode::SetContextU128
            | ContextOpcode::SetErgsPerPubdataByte
            | ContextOpcode::IncrementTxNumber => vec![],
            _ => vec![Operand::RegOnly],
        }
    }
    fn requires_kernel_mode(&self) -> bool {
        match self {
            ContextOpcode::SetContextU128
            | ContextOpcode::SetErgsPerPubdataByte
            | ContextOpcode::IncrementTxNumber => true,
            _ => false,
        }
    }
    fn can_be_used_in_static_context(&self) -> bool {
        match self {
            ContextOpcode::SetContextU128
            | ContextOpcode::SetErgsPerPubdataByte
            | ContextOpcode::IncrementTxNumber => false,
            _ => true,
        }
    }
}
