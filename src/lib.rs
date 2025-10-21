//! Fast Binary Encoding (FBE) runtime library for Rust
//! 
//! Based on the original FBE architecture from https://github.com/chronoxor/FastBinaryEncoding
//! 
//! HERSEY DAHA IYI BIR PANILUX ICIN! 🚀

pub mod buffer;
pub mod field_model;
pub mod field_model_collections;
pub mod model;

pub use buffer::{ReadBuffer, WriteBuffer};
pub use field_model::FieldModel;
pub use model::Model;

