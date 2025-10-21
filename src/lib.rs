//! Fast Binary Encoding (FBE) runtime library for Rust
//!
//! Based on the original FBE architecture from https://github.com/chronoxor/FastBinaryEncoding

pub mod address;
pub mod buffer;
pub mod defaults;
pub mod field_model;
pub mod field_model_collections;
pub mod inheritance;
pub mod keys;
pub mod model;
pub mod model_final;
pub mod user_with_address;

pub use buffer::{ReadBuffer, WriteBuffer};
pub use field_model::FieldModel;
pub use model::Model;
