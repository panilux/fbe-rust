//! Side enum

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i8)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}
