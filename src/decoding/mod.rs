use crate::DecodedOpcode;
use ethereum_types::U256;

pub mod encoding_mode_production;
pub mod encoding_mode_testing;

pub use self::encoding_mode_production::EncodingModeProduction;
pub use self::encoding_mode_testing::EncodingModeTesting;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariantMonotonicNumber(pub usize);

impl VariantMonotonicNumber {
    #[inline]
    pub const fn from_usize(value: usize) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn into_usize(self) -> usize {
        self.0 as usize
    }
}

pub trait AllowedIntegerRepresentation<const N: usize>:
    Clone + Copy + Send + Sync + PartialEq + Eq + Default + std::fmt::Debug + std::fmt::LowerHex
{
    fn encode_as_bytes(self) -> [u8; N];
}

impl AllowedIntegerRepresentation<16> for u128 {
    fn encode_as_bytes(self) -> [u8; 16] {
        self.to_be_bytes()
    }
}

impl AllowedIntegerRepresentation<8> for u64 {
    fn encode_as_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}

pub trait AllowedPcOrImm:
    Clone + Copy + Send + Sync + PartialEq + Eq + Default + std::fmt::Debug + std::fmt::LowerHex
{
    fn from_u64_clipped(value: u64) -> Self;
    fn as_u64(self) -> u64;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn max() -> Self;
}

impl AllowedPcOrImm for u16 {
    #[inline]
    fn from_u64_clipped(value: u64) -> Self {
        value as Self
    }
    #[inline]
    fn as_u64(self) -> u64 {
        self as u64
    }
    #[inline]
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    #[inline]
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    #[inline]
    fn max() -> Self {
        u16::MAX
    }
}
impl AllowedPcOrImm for u32 {
    #[inline]
    fn from_u64_clipped(value: u64) -> Self {
        value as Self
    }
    #[inline]
    fn as_u64(self) -> u64 {
        self as u64
    }
    #[inline]
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    #[inline]
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    #[inline]
    fn max() -> Self {
        u32::MAX
    }
}

// Some abstraction over encoding mode
// to be able to parse integer representation, as well as memory words
pub trait VmEncodingMode<const N: usize>:
    Clone + Copy + Send + Sync + PartialEq + Eq + std::fmt::Debug
{
    type PcOrImm: AllowedPcOrImm;
    type IntegerRepresentation: AllowedIntegerRepresentation<N>;

    fn nop_encoding() -> Self::IntegerRepresentation;
    fn exception_revert_encoding() -> Self::IntegerRepresentation;

    fn parse_preliminary_variant_and_absolute_number(
        integer_representaiton: Self::IntegerRepresentation,
    ) -> (DecodedOpcode<N, Self>, VariantMonotonicNumber);

    fn encode_as_integer(opcode: &DecodedOpcode<N, Self>) -> Self::IntegerRepresentation;

    fn encode_as_bytes(opcode: &DecodedOpcode<N, Self>) -> [u8; N];

    fn split_pc(pc: Self::PcOrImm) -> (Self::PcOrImm, Self::PcOrImm);

    fn integer_representaiton_from_u256(
        value: U256,
        index: Self::PcOrImm,
    ) -> Self::IntegerRepresentation;

    // there may be some unused bits, but we want canonicality
    fn is_canonical_encoding(value: Self::IntegerRepresentation) -> bool;
}
