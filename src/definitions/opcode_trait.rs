use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand {
    RegOnly,
    RegOrImm(RegOrImmFlags),
    Full(ImmMemHandlerFlags),
}
impl Operand {
    pub const fn variant_idx(&self) -> usize {
        match self {
            Operand::RegOnly => 0,
            Operand::RegOrImm(v) => v.variant_index(),
            Operand::Full(v) => v.variant_index(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ISAVersion(pub u8);

impl ISAVersion {
    pub const fn as_index(&self) -> usize {
        self.0 as usize
    }

    pub fn all_backward_compatible_versions(including: Self) -> Vec<Self> {
        let mut result = vec![];
        for v in ALL_ISA_VERSIONS.iter() {
            if v <= &including {
                result.push(*v);
            }
        }

        result
    }
}

pub const NUM_ISA_VERSIONS: usize = 2;
pub const ALL_ISA_VERSIONS: [ISAVersion; NUM_ISA_VERSIONS] = [ISAVersion(0), ISAVersion(1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpcodeVariantData {
    pub variant_idx: usize,
    pub num_non_exclusive_flags: usize,
    pub num_used_immediates: usize,
}

// Extra traits are for lazy statics
pub trait OpcodeProps: 'static + Send + Sync {
    fn name(&self) -> &'static str;
    fn variants_data(&self, version: ISAVersion) -> Vec<OpcodeVariantData>;
    fn max_variant_idx(&self, version: ISAVersion) -> usize;
    // addressing modes can not change from version to version
    fn input_operands(&self, version: ISAVersion) -> Vec<Operand>;
    fn output_operands(&self, version: ISAVersion) -> Vec<Operand>;
    // These are aux flags that indicate extra properties
    fn requires_kernel_mode(&self) -> bool;
    fn can_be_used_in_static_context(&self) -> bool;
    fn can_have_src0_from_mem(&self, version: ISAVersion) -> bool {
        let possible_dests = self.input_operands(version);

        if possible_dests.len() == 0 {
            false
        } else {
            match possible_dests[0] {
                Operand::Full(_) => true,
                Operand::RegOnly => false,
                Operand::RegOrImm(_) => false,
            }
        }
    }
    fn can_write_dst0_into_memory(&self, version: ISAVersion) -> bool {
        let possible_dests = self.output_operands(version);

        if possible_dests.len() == 0 {
            false
        } else {
            match possible_dests[0] {
                Operand::Full(_) => true,
                Operand::RegOnly => false,
                Operand::RegOrImm(_) => unreachable!(),
            }
        }
    }
}

pub trait OpcodeVariantProps: Sized + 'static + Send + Sync {
    fn all_variants() -> Vec<Self>;
    fn minimal_version(&self) -> ISAVersion;
    fn max_variant_idx_for_version(version: ISAVersion) -> usize;
    fn is_available_for_version(&self, version: ISAVersion) -> bool {
        self.minimal_version() >= version
    }
    fn is_added_in_version(&self, version: ISAVersion) -> bool {
        self.minimal_version() == version
    }
    fn variants_for_version(version: ISAVersion) -> Vec<Self> {
        Self::all_variants()
            .into_iter()
            .filter(|el| el.is_available_for_version(version))
            .collect()
    }
    fn variants_added_in_version(version: ISAVersion) -> Vec<Self> {
        Self::all_variants()
            .into_iter()
            .filter(|el| el.is_added_in_version(version))
            .collect()
    }
    fn variant_index(&self) -> usize;
    fn from_variant_index_for_version(index: usize, version: &ISAVersion) -> Option<Self>;
    fn ergs_price(&self) -> u32;
}

use std::ops::RangeInclusive;

pub(crate) fn full_variants_product(
    variants_range: RangeInclusive<usize>,
    num_non_exclusive_flags: usize,
    num_immedates: usize,
) -> Vec<OpcodeVariantData> {
    let mut result = Vec::with_capacity(variants_range.clone().count());
    for idx in variants_range {
        let descr = OpcodeVariantData {
            variant_idx: idx,
            num_non_exclusive_flags: num_non_exclusive_flags,
            num_used_immediates: num_immedates,
        };
        result.push(descr);
    }

    result
}
