#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Condition {
    Always = 0,
    Gt,
    Lt,
    Eq,
    Ge,
    Le,
    Ne,
    GtOrLt,
}

pub const CONDITIONAL_BITS_WIDTH: usize = 3;

impl Condition {
    const NUM_VARIANTS: usize = 8;

    pub fn materialize_variant(idx: usize) -> Self {
        assert!(idx < Self::NUM_VARIANTS);

        match idx {
            0 => Condition::Always,
            1 => Condition::Gt,
            2 => Condition::Lt,
            3 => Condition::Eq,
            4 => Condition::Ge,
            5 => Condition::Le,
            6 => Condition::Ne,
            7 => Condition::GtOrLt,
            _ => panic!("unknown variant"),
        }
    }

    pub const fn variant_index(&self) -> usize {
        match self {
            Condition::Always => 0,
            Condition::Gt => 1,
            Condition::Lt => 2,
            Condition::Eq => 3,
            Condition::Ge => 4,
            Condition::Le => 5,
            Condition::Ne => 6,
            Condition::GtOrLt => 7,
        }
    }
}

pub const ALL_CONDITIONS: [Condition; 1 << CONDITIONAL_BITS_WIDTH] = [
    Condition::Always,
    Condition::Gt,
    Condition::Lt,
    Condition::Eq,
    Condition::Ge,
    Condition::Le,
    Condition::Ne,
    Condition::GtOrLt,
];
