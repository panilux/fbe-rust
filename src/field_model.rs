//! Fast Binary Encoding field models
//!
//! Field model pattern for type-safe serialization/deserialization.
//! Following original FBE design with Rust zero-cost abstractions.

use crate::buffer::{ReadBuffer, WriteBuffer};

/// Base trait for all field models
pub trait FieldModel {
    /// Get field offset in buffer
    fn offset(&self) -> usize;

    /// Set field offset in buffer
    fn set_offset(&mut self, offset: usize);

    /// Get field size in bytes
    fn size(&self) -> usize;

    /// Get extra size for dynamic types (strings, vectors, etc.)
    fn extra(&self) -> usize {
        0
    }

    /// Shift offset forward
    fn shift(&mut self, size: usize) {
        let current = self.offset();
        self.set_offset(current + size);
    }

    /// Shift offset backward
    fn unshift(&mut self, size: usize) {
        let current = self.offset();
        self.set_offset(current - size);
    }

    /// Verify field value
    fn verify(&self) -> bool {
        true
    }
}

// ============================================================================
// Macro for primitive field models
// ============================================================================

macro_rules! impl_primitive_field_model {
    ($name:ident, $name_mut:ident, $type:ty, $size:expr, $read_fn:ident, $write_fn:ident) => {
        pub struct $name<'a> {
            buffer: &'a [u8],
            offset: usize,
        }

        impl<'a> $name<'a> {
            pub fn new(buffer: &'a [u8], offset: usize) -> Self {
                Self { buffer, offset }
            }

            pub fn get(&self) -> $type {
                ReadBuffer::from(self.buffer.to_vec()).$read_fn(self.offset)
            }
        }

        impl<'a> FieldModel for $name<'a> {
            fn offset(&self) -> usize {
                self.offset
            }
            fn set_offset(&mut self, offset: usize) {
                self.offset = offset;
            }
            fn size(&self) -> usize {
                $size
            }
        }

        pub struct $name_mut<'a> {
            buffer: &'a mut WriteBuffer,
            offset: usize,
        }

        impl<'a> $name_mut<'a> {
            pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
                Self { buffer, offset }
            }

            pub fn set(&mut self, value: $type) {
                self.buffer.$write_fn(self.offset, value);
            }
        }

        impl<'a> FieldModel for $name_mut<'a> {
            fn offset(&self) -> usize {
                self.offset
            }
            fn set_offset(&mut self, offset: usize) {
                self.offset = offset;
            }
            fn size(&self) -> usize {
                $size
            }
        }
    };
}

// ============================================================================
// Primitive Types
// ============================================================================

impl_primitive_field_model!(
    FieldModelBool,
    FieldModelBoolMut,
    bool,
    1,
    read_bool,
    write_bool
);
impl_primitive_field_model!(FieldModelI8, FieldModelI8Mut, i8, 1, read_i8, write_i8);
impl_primitive_field_model!(FieldModelI16, FieldModelI16Mut, i16, 2, read_i16, write_i16);
impl_primitive_field_model!(FieldModelI32, FieldModelI32Mut, i32, 4, read_i32, write_i32);
impl_primitive_field_model!(FieldModelI64, FieldModelI64Mut, i64, 8, read_i64, write_i64);
impl_primitive_field_model!(FieldModelU8, FieldModelU8Mut, u8, 1, read_u8, write_u8);
impl_primitive_field_model!(FieldModelU16, FieldModelU16Mut, u16, 2, read_u16, write_u16);
impl_primitive_field_model!(FieldModelU32, FieldModelU32Mut, u32, 4, read_u32, write_u32);
impl_primitive_field_model!(FieldModelU64, FieldModelU64Mut, u64, 8, read_u64, write_u64);
impl_primitive_field_model!(FieldModelF32, FieldModelF32Mut, f32, 4, read_f32, write_f32);
impl_primitive_field_model!(FieldModelF64, FieldModelF64Mut, f64, 8, read_f64, write_f64);

// ============================================================================
// String
// ============================================================================

pub struct FieldModelString<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelString<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> String {
        ReadBuffer::from(self.buffer.to_vec()).read_string(self.offset)
    }
}

impl<'a> FieldModel for FieldModelString<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    } // Size prefix only

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let len_bytes = &self.buffer[self.offset..self.offset + 4];
        u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize
    }
}

pub struct FieldModelStringMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelStringMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: &str) {
        self.buffer.write_string(self.offset, value);
    }
}

impl<'a> FieldModel for FieldModelStringMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    }
}

// ============================================================================
// Timestamp
// ============================================================================

impl_primitive_field_model!(
    FieldModelTimestamp,
    FieldModelTimestampMut,
    u64,
    8,
    read_timestamp,
    write_timestamp
);

// ============================================================================
// UUID
// ============================================================================

pub struct FieldModelUuid<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelUuid<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> String {
        let bytes = ReadBuffer::from(self.buffer.to_vec()).read_uuid(self.offset);
        // Convert binary to UUID string
        format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15])
    }
}

impl<'a> FieldModel for FieldModelUuid<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        16
    }
}

pub struct FieldModelUuidMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelUuidMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: &str) {
        // Parse UUID string to binary
        let uuid_str = value.replace("-", "");
        let mut bytes = [0u8; 16];
        for i in 0..16 {
            bytes[i] = u8::from_str_radix(&uuid_str[i * 2..i * 2 + 2], 16).unwrap_or(0);
        }
        self.buffer.write_uuid(self.offset, &bytes);
    }
}

impl<'a> FieldModel for FieldModelUuidMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        16
    }
}

// ============================================================================
// Bytes
// ============================================================================

pub struct FieldModelBytes<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelBytes<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> Vec<u8> {
        ReadBuffer::from(self.buffer.to_vec()).read_bytes(self.offset)
    }
}

impl<'a> FieldModel for FieldModelBytes<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    } // Size prefix only

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let len_bytes = &self.buffer[self.offset..self.offset + 4];
        u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize
    }
}

pub struct FieldModelBytesMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelBytesMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: &[u8]) {
        self.buffer.write_bytes(self.offset, value);
    }
}

impl<'a> FieldModel for FieldModelBytesMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    }
}

// ============================================================================
// Decimal
// ============================================================================

pub struct FieldModelDecimal<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelDecimal<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> (i128, u8, bool) {
        ReadBuffer::from(self.buffer.to_vec()).read_decimal(self.offset)
    }
}

impl<'a> FieldModel for FieldModelDecimal<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        16
    }
}

pub struct FieldModelDecimalMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelDecimalMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: i128, scale: u8, negative: bool) {
        self.buffer
            .write_decimal(self.offset, value, scale, negative);
    }
}

impl<'a> FieldModel for FieldModelDecimalMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        16
    }
}
