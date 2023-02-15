use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Invalid(InvalidOpcode),
    Nop(NopOpcode),
    Add(AddOpcode),
    Sub(SubOpcode),
    Mul(MulOpcode),
    Div(DivOpcode),
    Jump(JumpOpcode),
    Context(ContextOpcode),
    Shift(ShiftOpcode),
    Binop(BinopOpcode),
    Ptr(PtrOpcode),
    NearCall(NearCallOpcode),
    Log(LogOpcode),
    FarCall(FarCallOpcode),
    Ret(RetOpcode),
    UMA(UMAOpcode),
}

impl Opcode {
    pub fn ergs_price(&self) -> u32 {
        match self {
            Opcode::Invalid(inner) => inner.ergs_price(),
            Opcode::Nop(inner) => inner.ergs_price(),
            Opcode::Add(inner) => inner.ergs_price(),
            Opcode::Sub(inner) => inner.ergs_price(),
            Opcode::Mul(inner) => inner.ergs_price(),
            Opcode::Div(inner) => inner.ergs_price(),
            Opcode::Jump(inner) => inner.ergs_price(),
            Opcode::Context(inner) => inner.ergs_price(),
            Opcode::Shift(inner) => inner.ergs_price(),
            Opcode::Binop(inner) => inner.ergs_price(),
            Opcode::Ptr(inner) => inner.ergs_price(),
            Opcode::NearCall(inner) => inner.ergs_price(),
            Opcode::Log(inner) => inner.ergs_price(),
            Opcode::FarCall(inner) => inner.ergs_price(),
            Opcode::Ret(inner) => inner.ergs_price(),
            Opcode::UMA(inner) => inner.ergs_price(),
        }
    }

    pub fn materialize_subvariant_from_prototype(&self, idx: usize, version: &ISAVersion) -> Self {
        match self {
            Opcode::Invalid(_) => Opcode::Invalid(
                InvalidOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Nop(_) => Opcode::Nop(
                NopOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Add(_) => Opcode::Add(
                AddOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Sub(_) => Opcode::Sub(
                SubOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Mul(_) => Opcode::Mul(
                MulOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Div(_) => Opcode::Div(
                DivOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Jump(_) => Opcode::Jump(
                JumpOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::Context(_) => Opcode::Context(
                ContextOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Shift(_) => Opcode::Shift(
                ShiftOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Binop(_) => Opcode::Binop(
                BinopOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Ptr(_) => Opcode::Ptr(
                PtrOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::NearCall(_) => Opcode::NearCall(
                NearCallOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Log(_) => Opcode::Log(
                LogOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::FarCall(_) => Opcode::FarCall(
                FarCallOpcode::from_variant_index_for_version(idx, version)
                    .expect("must materialize"),
            ),
            Opcode::Ret(_) => Opcode::Ret(
                RetOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
            Opcode::UMA(_) => Opcode::UMA(
                UMAOpcode::from_variant_index_for_version(idx, version).expect("must materialize"),
            ),
        }
    }

    pub const fn variant_idx(&self) -> usize {
        match self {
            Opcode::Invalid(_) => 0,
            Opcode::Nop(_) => 1,
            Opcode::Add(_) => 2,
            Opcode::Sub(_) => 3,
            Opcode::Mul(_) => 4,
            Opcode::Div(_) => 5,
            Opcode::Jump(_) => 6,
            Opcode::Context(_) => 7,
            Opcode::Shift(_) => 8,
            Opcode::Binop(_) => 9,
            Opcode::Ptr(_) => 10,
            Opcode::NearCall(_) => 11,
            Opcode::Log(_) => 12,
            Opcode::FarCall(_) => 13,
            Opcode::Ret(_) => 14,
            Opcode::UMA(_) => 15,
        }
    }

    pub fn materialize_subvariant_idx(&self) -> usize {
        match self {
            Opcode::Invalid(sub) => sub.variant_index(),
            Opcode::Nop(sub) => sub.variant_index(),
            Opcode::Add(sub) => sub.variant_index(),
            Opcode::Sub(sub) => sub.variant_index(),
            Opcode::Mul(sub) => sub.variant_index(),
            Opcode::Div(sub) => sub.variant_index(),
            Opcode::Jump(sub) => sub.variant_index(),
            Opcode::Context(sub) => sub.variant_index(),
            Opcode::Shift(sub) => sub.variant_index(),
            Opcode::Binop(sub) => sub.variant_index(),
            Opcode::Ptr(sub) => sub.variant_index(),
            Opcode::NearCall(sub) => sub.variant_index(),
            Opcode::Log(sub) => sub.variant_index(),
            Opcode::FarCall(sub) => sub.variant_index(),
            Opcode::Ret(sub) => sub.variant_index(),
            Opcode::UMA(sub) => sub.variant_index(),
        }
    }

    pub fn requires_kernel_mode(&self) -> bool {
        match self {
            Opcode::Invalid(sub) => sub.requires_kernel_mode(),
            Opcode::Nop(sub) => sub.requires_kernel_mode(),
            Opcode::Add(sub) => sub.requires_kernel_mode(),
            Opcode::Sub(sub) => sub.requires_kernel_mode(),
            Opcode::Mul(sub) => sub.requires_kernel_mode(),
            Opcode::Div(sub) => sub.requires_kernel_mode(),
            Opcode::Jump(sub) => sub.requires_kernel_mode(),
            Opcode::Context(sub) => sub.requires_kernel_mode(),
            Opcode::Shift(sub) => sub.requires_kernel_mode(),
            Opcode::Binop(sub) => sub.requires_kernel_mode(),
            Opcode::Ptr(sub) => sub.requires_kernel_mode(),
            Opcode::NearCall(sub) => sub.requires_kernel_mode(),
            Opcode::Log(sub) => sub.requires_kernel_mode(),
            Opcode::FarCall(sub) => sub.requires_kernel_mode(),
            Opcode::Ret(sub) => sub.requires_kernel_mode(),
            Opcode::UMA(sub) => sub.requires_kernel_mode(),
        }
    }

    pub fn can_be_used_in_static_context(&self) -> bool {
        match self {
            Opcode::Invalid(sub) => sub.can_be_used_in_static_context(),
            Opcode::Nop(sub) => sub.can_be_used_in_static_context(),
            Opcode::Add(sub) => sub.can_be_used_in_static_context(),
            Opcode::Sub(sub) => sub.can_be_used_in_static_context(),
            Opcode::Mul(sub) => sub.can_be_used_in_static_context(),
            Opcode::Div(sub) => sub.can_be_used_in_static_context(),
            Opcode::Jump(sub) => sub.can_be_used_in_static_context(),
            Opcode::Context(sub) => sub.can_be_used_in_static_context(),
            Opcode::Shift(sub) => sub.can_be_used_in_static_context(),
            Opcode::Binop(sub) => sub.can_be_used_in_static_context(),
            Opcode::Ptr(sub) => sub.can_be_used_in_static_context(),
            Opcode::NearCall(sub) => sub.can_be_used_in_static_context(),
            Opcode::Log(sub) => sub.can_be_used_in_static_context(),
            Opcode::FarCall(sub) => sub.can_be_used_in_static_context(),
            Opcode::Ret(sub) => sub.can_be_used_in_static_context(),
            Opcode::UMA(sub) => sub.can_be_used_in_static_context(),
        }
    }

    pub fn can_have_src0_from_mem(&self, version: ISAVersion) -> bool {
        match self {
            Opcode::Invalid(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Nop(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Add(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Sub(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Mul(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Div(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Jump(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Context(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Shift(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Binop(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Ptr(sub) => sub.can_have_src0_from_mem(version),
            Opcode::NearCall(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Log(sub) => sub.can_have_src0_from_mem(version),
            Opcode::FarCall(sub) => sub.can_have_src0_from_mem(version),
            Opcode::Ret(sub) => sub.can_have_src0_from_mem(version),
            Opcode::UMA(sub) => sub.can_have_src0_from_mem(version),
        }
    }

    pub fn can_write_dst0_into_memory(&self, version: ISAVersion) -> bool {
        match self {
            Opcode::Invalid(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Nop(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Add(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Sub(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Mul(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Div(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Jump(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Context(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Shift(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Binop(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Ptr(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::NearCall(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Log(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::FarCall(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::Ret(sub) => sub.can_write_dst0_into_memory(version),
            Opcode::UMA(sub) => sub.can_write_dst0_into_memory(version),
        }
    }

    pub fn input_operands(&self, version: ISAVersion) -> Vec<Operand> {
        match self {
            Opcode::Invalid(sub) => sub.input_operands(version),
            Opcode::Nop(sub) => sub.input_operands(version),
            Opcode::Add(sub) => sub.input_operands(version),
            Opcode::Sub(sub) => sub.input_operands(version),
            Opcode::Mul(sub) => sub.input_operands(version),
            Opcode::Div(sub) => sub.input_operands(version),
            Opcode::Jump(sub) => sub.input_operands(version),
            Opcode::Context(sub) => sub.input_operands(version),
            Opcode::Shift(sub) => sub.input_operands(version),
            Opcode::Binop(sub) => sub.input_operands(version),
            Opcode::Ptr(sub) => sub.input_operands(version),
            Opcode::NearCall(sub) => sub.input_operands(version),
            Opcode::Log(sub) => sub.input_operands(version),
            Opcode::FarCall(sub) => sub.input_operands(version),
            Opcode::Ret(sub) => sub.input_operands(version),
            Opcode::UMA(sub) => sub.input_operands(version),
        }
    }

    pub fn output_operands(&self, version: ISAVersion) -> Vec<Operand> {
        match self {
            Opcode::Invalid(sub) => sub.output_operands(version),
            Opcode::Nop(sub) => sub.output_operands(version),
            Opcode::Add(sub) => sub.output_operands(version),
            Opcode::Sub(sub) => sub.output_operands(version),
            Opcode::Mul(sub) => sub.output_operands(version),
            Opcode::Div(sub) => sub.output_operands(version),
            Opcode::Jump(sub) => sub.output_operands(version),
            Opcode::Context(sub) => sub.output_operands(version),
            Opcode::Shift(sub) => sub.output_operands(version),
            Opcode::Binop(sub) => sub.output_operands(version),
            Opcode::Ptr(sub) => sub.output_operands(version),
            Opcode::NearCall(sub) => sub.output_operands(version),
            Opcode::Log(sub) => sub.output_operands(version),
            Opcode::FarCall(sub) => sub.output_operands(version),
            Opcode::Ret(sub) => sub.output_operands(version),
            Opcode::UMA(sub) => sub.output_operands(version),
        }
    }
}
