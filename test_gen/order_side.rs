//! OrderSide enum

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum OrderSide {
    Buy,
    Sell,
}
