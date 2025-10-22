//! Fast Binary Encoding final models (compact, inline format)
//!
//! FinalModel is the compact version without versioning support.
//! All data is inline, no pointers.

use crate::buffer::{ReadBuffer, WriteBuffer};

/// Base trait for all final models
pub trait FinalModel {
    /// Get field offset in buffer
    fn offset(&self) -> usize;

    /// Set field offset in buffer
    fn set_offset(&mut self, offset: usize);

    /// Get field size in bytes
    fn size(&self) -> usize;

    /// Get extra size for dynamic types
    fn extra(&self) -> usize {
        0
    }
}

// ============================================================================
// Macro for primitive final models
// ============================================================================

macro_rules! impl_primitive_final_model {
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

        impl<'a> FinalModel for $name<'a> {
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

        impl<'a> FinalModel for $name_mut<'a> {
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

impl_primitive_final_model!(
    FinalModelBool,
    FinalModelBoolMut,
    bool,
    1,
    read_bool,
    write_bool
);
impl_primitive_final_model!(FinalModelByte, FinalModelByteMut, u8, 1, read_byte, write_byte);
impl_primitive_final_model!(FinalModelChar, FinalModelCharMut, u8, 1, read_char, write_char);
impl_primitive_final_model!(FinalModelWChar, FinalModelWCharMut, u32, 4, read_wchar, write_wchar);
impl_primitive_final_model!(FinalModelI8, FinalModelI8Mut, i8, 1, read_i8, write_i8);
impl_primitive_final_model!(FinalModelI16, FinalModelI16Mut, i16, 2, read_i16, write_i16);
impl_primitive_final_model!(FinalModelI32, FinalModelI32Mut, i32, 4, read_i32, write_i32);
impl_primitive_final_model!(FinalModelI64, FinalModelI64Mut, i64, 8, read_i64, write_i64);
impl_primitive_final_model!(FinalModelU8, FinalModelU8Mut, u8, 1, read_u8, write_u8);
impl_primitive_final_model!(FinalModelU16, FinalModelU16Mut, u16, 2, read_u16, write_u16);
impl_primitive_final_model!(FinalModelU32, FinalModelU32Mut, u32, 4, read_u32, write_u32);
impl_primitive_final_model!(FinalModelU64, FinalModelU64Mut, u64, 8, read_u64, write_u64);
impl_primitive_final_model!(FinalModelF32, FinalModelF32Mut, f32, 4, read_f32, write_f32);
impl_primitive_final_model!(FinalModelF64, FinalModelF64Mut, f64, 8, read_f64, write_f64);

// ============================================================================
// String (inline format: 4-byte size + data)
// ============================================================================

pub struct FinalModelString<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FinalModelString<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> String {
        ReadBuffer::from(self.buffer.to_vec()).read_string(self.offset)
    }
}

impl<'a> FinalModel for FinalModelString<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 4;
        }
        let len_bytes = &self.buffer[self.offset..self.offset + 4];
        let len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
        4 + len
    }
}

pub struct FinalModelStringMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FinalModelStringMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: &str) {
        self.buffer.write_string(self.offset, value);
    }
}

impl<'a> FinalModel for FinalModelStringMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4 // Size prefix
    }
}

// ============================================================================
// Bytes (inline format: 4-byte size + data)
// ============================================================================

pub struct FinalModelBytes<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FinalModelBytes<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> Vec<u8> {
        ReadBuffer::from(self.buffer.to_vec()).read_bytes(self.offset)
    }
}

impl<'a> FinalModel for FinalModelBytes<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 4;
        }
        let len_bytes = &self.buffer[self.offset..self.offset + 4];
        let len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
        4 + len
    }
}

// ============================================================================
// Decimal (inline format: 16 bytes)
// ============================================================================

pub struct FinalModelDecimal<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FinalModelDecimal<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> (i128, u8, bool) {
        ReadBuffer::from(self.buffer.to_vec()).read_decimal(self.offset)
    }
}

impl<'a> FinalModel for FinalModelDecimal<'a> {
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

pub struct FinalModelDecimalMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FinalModelDecimalMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, value: i128, scale: u8, negative: bool) {
        self.buffer.write_decimal(self.offset, value, scale, negative);
    }
}

impl<'a> FinalModel for FinalModelDecimalMut<'a> {
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
// Timestamp (inline format: 8 bytes)
// ============================================================================

impl_primitive_final_model!(
    FinalModelTimestamp,
    FinalModelTimestampMut,
    u64,
    8,
    read_timestamp,
    write_timestamp
);

// ============================================================================
// UUID (inline format: 16 bytes)
// ============================================================================

pub struct FinalModelUuid<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FinalModelUuid<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> String {
        let bytes = ReadBuffer::from(self.buffer.to_vec()).read_uuid(self.offset);
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }
}

impl<'a> FinalModel for FinalModelUuid<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_model_primitives() {
        let mut writer = WriteBuffer::new();
        writer.allocate(13); // Reserve space
        writer.write_bool(0, true);
        writer.write_i32(1, 42);
        writer.write_f64(5, 3.14159);

        let reader_data = writer.data().to_vec();
        
        let bool_field = FinalModelBool::new(&reader_data, 0);
        assert_eq!(bool_field.get(), true);
        assert_eq!(bool_field.size(), 1);

        let int_field = FinalModelI32::new(&reader_data, 1);
        assert_eq!(int_field.get(), 42);
        assert_eq!(int_field.size(), 4);

        let double_field = FinalModelF64::new(&reader_data, 5);
        assert!((double_field.get() - 3.14159).abs() < 0.0001);
        assert_eq!(double_field.size(), 8);
    }

    #[test]
    fn test_final_model_string() {
        let mut writer = WriteBuffer::new();
        writer.allocate(9); // 4-byte size + 5 bytes data
        writer.write_string(0, "Hello");

        let reader_data = writer.data().to_vec();
        
        let string_field = FinalModelString::new(&reader_data, 0);
        assert_eq!(string_field.get(), "Hello");
        assert_eq!(string_field.size(), 4 + 5); // 4-byte size + 5 bytes data
    }
}


// ============================================================================
// Collection Final Models
// ============================================================================

/// FinalModel for vector<T> (inline format)
pub struct FinalModelVector<'a, T> {
    buffer: &'a [u8],
    offset: usize,
    item_model: fn(&'a [u8], usize) -> T,
}

impl<'a, T> FinalModelVector<'a, T> {
    pub fn new(buffer: &'a [u8], offset: usize, item_model: fn(&'a [u8], usize) -> T) -> Self {
        Self { buffer, offset, item_model }
    }

    pub fn get(&self) -> Vec<T> {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        
        let size = read_buf.read_u32(self.offset) as usize;
        let mut result = Vec::with_capacity(size);
        
        let mut item_offset = self.offset + 4;
        for _ in 0..size {
            result.push((self.item_model)(self.buffer, item_offset));
            item_offset += std::mem::size_of::<T>();
        }
        
        result
    }

    pub fn size(&self) -> usize {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        let size = read_buf.read_u32(self.offset) as usize;
        4 + size * std::mem::size_of::<T>()
    }
}

/// FinalModel for array<T, N> (inline, fixed size)
pub struct FinalModelArray<'a, T, const N: usize> {
    buffer: &'a [u8],
    offset: usize,
    item_model: fn(&'a [u8], usize) -> T,
}

impl<'a, T, const N: usize> FinalModelArray<'a, T, N> {
    pub fn new(buffer: &'a [u8], offset: usize, item_model: fn(&'a [u8], usize) -> T) -> Self {
        Self { buffer, offset, item_model }
    }

    pub fn get(&self) -> [T; N] {
        let mut result = Vec::with_capacity(N);
        let mut item_offset = self.offset;
        
        for _ in 0..N {
            result.push((self.item_model)(self.buffer, item_offset));
            item_offset += std::mem::size_of::<T>();
        }
        
        result.try_into().unwrap_or_else(|_| panic!("Array size mismatch"))
    }

    pub fn size(&self) -> usize {
        N * std::mem::size_of::<T>()
    }
}

/// FinalModel for map<K, V> (inline format)
pub struct FinalModelMap<'a, K, V> {
    buffer: &'a [u8],
    offset: usize,
    key_model: fn(&'a [u8], usize) -> K,
    value_model: fn(&'a [u8], usize) -> V,
}

impl<'a, K, V> FinalModelMap<'a, K, V> {
    pub fn new(
        buffer: &'a [u8],
        offset: usize,
        key_model: fn(&'a [u8], usize) -> K,
        value_model: fn(&'a [u8], usize) -> V,
    ) -> Self {
        Self { buffer, offset, key_model, value_model }
    }

    pub fn get(&self) -> std::collections::HashMap<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        
        let size = read_buf.read_u32(self.offset) as usize;
        let mut result = std::collections::HashMap::with_capacity(size);
        
        let mut item_offset = self.offset + 4;
        for _ in 0..size {
            let key = (self.key_model)(self.buffer, item_offset);
            item_offset += std::mem::size_of::<K>();
            let value = (self.value_model)(self.buffer, item_offset);
            item_offset += std::mem::size_of::<V>();
            result.insert(key, value);
        }
        
        result
    }

    pub fn size(&self) -> usize {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        let size = read_buf.read_u32(self.offset) as usize;
        4 + size * (std::mem::size_of::<K>() + std::mem::size_of::<V>())
    }
}

/// FinalModel for set<T> (inline format)
pub struct FinalModelSet<'a, T> {
    buffer: &'a [u8],
    offset: usize,
    item_model: fn(&'a [u8], usize) -> T,
}

impl<'a, T> FinalModelSet<'a, T> {
    pub fn new(buffer: &'a [u8], offset: usize, item_model: fn(&'a [u8], usize) -> T) -> Self {
        Self { buffer, offset, item_model }
    }

    pub fn get(&self) -> std::collections::HashSet<T>
    where
        T: std::hash::Hash + Eq,
    {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        
        let size = read_buf.read_u32(self.offset) as usize;
        let mut result = std::collections::HashSet::with_capacity(size);
        
        let mut item_offset = self.offset + 4;
        for _ in 0..size {
            result.insert((self.item_model)(self.buffer, item_offset));
            item_offset += std::mem::size_of::<T>();
        }
        
        result
    }

    pub fn size(&self) -> usize {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        let size = read_buf.read_u32(self.offset) as usize;
        4 + size * std::mem::size_of::<T>()
    }
}

/// FinalModel for list<T> (inline format)
pub struct FinalModelList<'a, T> {
    buffer: &'a [u8],
    offset: usize,
    item_model: fn(&'a [u8], usize) -> T,
}

impl<'a, T> FinalModelList<'a, T> {
    pub fn new(buffer: &'a [u8], offset: usize, item_model: fn(&'a [u8], usize) -> T) -> Self {
        Self { buffer, offset, item_model }
    }

    pub fn get(&self) -> std::collections::LinkedList<T> {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        
        let size = read_buf.read_u32(self.offset) as usize;
        let mut result = std::collections::LinkedList::new();
        
        let mut item_offset = self.offset + 4;
        for _ in 0..size {
            result.push_back((self.item_model)(self.buffer, item_offset));
            item_offset += std::mem::size_of::<T>();
        }
        
        result
    }

    pub fn size(&self) -> usize {
        let mut read_buf = crate::buffer::ReadBuffer::new();
        read_buf.attach_buffer(self.buffer, 0, self.buffer.len());
        let size = read_buf.read_u32(self.offset) as usize;
        4 + size * std::mem::size_of::<T>()
    }
}
