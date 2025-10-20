//! OrderType enum

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}
