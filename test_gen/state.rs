//! State flags

pub const UNKNOWN: i8 = 0x00;
pub const INVALID: i8 = 0x01;
pub const INITIALIZED: i8 = 0x02;
pub const CALCULATED: i8 = 0x04;
pub const BROKEN: i8 = 0x08;
pub const GOOD: i8 = initialized | calculated;
pub const BAD: i8 = unknown | invalid | broken;
