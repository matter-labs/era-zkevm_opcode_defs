#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImmMemHandlerFlags {
    UseRegOnly = 0,
    UseStackWithPushPop,
    UseStackWithOffset,
    UseAbsoluteOnStack,
    UseImm16Only,
    UseCodePage,
}

impl ImmMemHandlerFlags {
    const NUM_VARIANTS: usize = 6;

    pub const fn all_variants() -> [Self; 6] {
        [
            ImmMemHandlerFlags::UseRegOnly,
            ImmMemHandlerFlags::UseStackWithPushPop,
            ImmMemHandlerFlags::UseStackWithOffset,
            ImmMemHandlerFlags::UseAbsoluteOnStack,
            ImmMemHandlerFlags::UseImm16Only,
            ImmMemHandlerFlags::UseCodePage,
        ]
    }

    pub const fn encoding_byte(&self) -> u8 {
        *self as u8
    }
    pub const fn is_memory_used(&self) -> bool {
        match self {
            ImmMemHandlerFlags::UseRegOnly | ImmMemHandlerFlags::UseImm16Only => false,
            _ => true,
        }
    }
    pub fn num_variants() -> usize {
        Self::NUM_VARIANTS
    }
    pub fn num_src_variants() -> usize {
        Self::NUM_VARIANTS
    }
    pub fn num_dst_variants() -> usize {
        Self::NUM_VARIANTS - 2
    }
    // pub fn materialize_variant(idx: usize) -> Self {
    //     assert!(idx < Self::NUM_VARIANTS);

    //     match idx {
    //         i if i == ImmMemHandlerFlags::UseRegOnly.variant_index() => {
    //             ImmMemHandlerFlags::UseRegOnly
    //         }
    //         i if i == ImmMemHandlerFlags::UseStackWithPushPop.variant_index() => {
    //             ImmMemHandlerFlags::UseStackWithPushPop
    //         }
    //         i if i == ImmMemHandlerFlags::UseStackWithOffset.variant_index() => {
    //             ImmMemHandlerFlags::UseStackWithOffset
    //         }
    //         i if i == ImmMemHandlerFlags::UseAbsoluteOnStack.variant_index() => {
    //             ImmMemHandlerFlags::UseAbsoluteOnStack
    //         }
    //         i if i == ImmMemHandlerFlags::UseImm16Only.variant_index() => {
    //             ImmMemHandlerFlags::UseImm16Only
    //         }
    //         i if i == ImmMemHandlerFlags::UseCodePage.variant_index() => {
    //             ImmMemHandlerFlags::UseCodePage
    //         }
    //         _ => panic!("unknown variant"),
    //     }
    // }
    pub const fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }
    pub const fn is_allowed_for_dst(&self) -> bool {
        match self {
            ImmMemHandlerFlags::UseImm16Only | ImmMemHandlerFlags::UseCodePage => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegOrImmFlags {
    UseRegOnly = 0,
    UseImm16Only = 4,
}

impl RegOrImmFlags {
    const NUM_VARIANTS: usize = 2;

    pub const fn all_variants() -> [Self; 2] {
        [RegOrImmFlags::UseRegOnly, RegOrImmFlags::UseImm16Only]
    }

    pub const fn encoding_byte(&self) -> u8 {
        *self as u8
    }
    pub const fn is_memory_used(&self) -> bool {
        false
    }
    pub fn num_variants() -> usize {
        Self::NUM_VARIANTS
    }
    pub fn num_src_variants() -> usize {
        Self::NUM_VARIANTS
    }
    pub fn num_dst_variants() -> usize {
        Self::NUM_VARIANTS - 1
    }
    pub const fn variant_index(&self) -> usize {
        (*self as u8) as usize
    }
    pub const fn is_allowed_for_dst(&self) -> bool {
        match self {
            RegOrImmFlags::UseImm16Only => false,
            _ => true,
        }
    }
}

const _: () = if ImmMemHandlerFlags::UseRegOnly.variant_index()
    != RegOrImmFlags::UseRegOnly.variant_index()
{
    panic!()
};
const _: () = if ImmMemHandlerFlags::UseImm16Only.variant_index()
    != RegOrImmFlags::UseImm16Only.variant_index()
{
    panic!()
};
