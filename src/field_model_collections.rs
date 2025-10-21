//! Fast Binary Encoding collection field models
//!
//! Collection field models: Vector, Array, Map, Set

use crate::buffer::{ReadBuffer, WriteBuffer};
use crate::field_model::FieldModel;

// ============================================================================
// Vector<T> - Dynamic array
// ============================================================================

pub struct FieldModelVectorI32<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelVectorI32<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> Vec<i32> {
        ReadBuffer::from(self.buffer.to_vec()).read_vector_i32(self.offset)
    }
}

impl<'a> FieldModel for FieldModelVectorI32<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    } // Pointer only

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let pointer = u32::from_le_bytes([
            self.buffer[self.offset],
            self.buffer[self.offset + 1],
            self.buffer[self.offset + 2],
            self.buffer[self.offset + 3],
        ]) as usize;

        if pointer == 0 || self.buffer.len() < pointer + 4 {
            return 0;
        }

        let count = u32::from_le_bytes([
            self.buffer[pointer],
            self.buffer[pointer + 1],
            self.buffer[pointer + 2],
            self.buffer[pointer + 3],
        ]) as usize;

        4 + (count * 4) // size + elements
    }
}

pub struct FieldModelVectorI32Mut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelVectorI32Mut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, values: &[i32]) {
        self.buffer.write_vector_i32(self.offset, values);
    }
}

impl<'a> FieldModel for FieldModelVectorI32Mut<'a> {
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
// Array[N] - Fixed-size array
// ============================================================================

pub struct FieldModelArrayI32<'a> {
    buffer: &'a [u8],
    offset: usize,
    count: usize,
}

impl<'a> FieldModelArrayI32<'a> {
    pub fn new(buffer: &'a [u8], offset: usize, count: usize) -> Self {
        Self {
            buffer,
            offset,
            count,
        }
    }

    pub fn get(&self) -> Vec<i32> {
        ReadBuffer::from(self.buffer.to_vec()).read_array_i32(self.offset, self.count)
    }
}

impl<'a> FieldModel for FieldModelArrayI32<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        self.count * 4
    }
}

pub struct FieldModelArrayI32Mut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
    count: usize,
}

impl<'a> FieldModelArrayI32Mut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize, count: usize) -> Self {
        Self {
            buffer,
            offset,
            count,
        }
    }

    pub fn set(&mut self, values: &[i32]) {
        if values.len() != self.count {
            panic!(
                "Array size mismatch: expected {}, got {}",
                self.count,
                values.len()
            );
        }
        self.buffer.write_array_i32(self.offset, values);
    }
}

impl<'a> FieldModel for FieldModelArrayI32Mut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        self.count * 4
    }
}

// ============================================================================
// Map<K,V> - Key-value pairs
// ============================================================================

use std::collections::HashMap;

pub struct FieldModelMapI32<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelMapI32<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> HashMap<i32, i32> {
        let pairs = ReadBuffer::from(self.buffer.to_vec()).read_map_i32(self.offset);
        pairs.into_iter().collect()
    }
}

impl<'a> FieldModel for FieldModelMapI32<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    } // Pointer only

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let pointer = u32::from_le_bytes([
            self.buffer[self.offset],
            self.buffer[self.offset + 1],
            self.buffer[self.offset + 2],
            self.buffer[self.offset + 3],
        ]) as usize;

        if pointer == 0 || self.buffer.len() < pointer + 4 {
            return 0;
        }

        let count = u32::from_le_bytes([
            self.buffer[pointer],
            self.buffer[pointer + 1],
            self.buffer[pointer + 2],
            self.buffer[pointer + 3],
        ]) as usize;

        4 + (count * 8) // size + (key+value pairs)
    }
}

pub struct FieldModelMapI32Mut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelMapI32Mut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, map: &HashMap<i32, i32>) {
        // Convert HashMap to vec of (key, value) pairs
        let pairs: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        self.buffer.write_map_i32(self.offset, &pairs);
    }
}

impl<'a> FieldModel for FieldModelMapI32Mut<'a> {
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
// Set<T> - Unique values
// ============================================================================

use std::collections::HashSet;

pub struct FieldModelSetI32<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelSetI32<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> HashSet<i32> {
        let vec = ReadBuffer::from(self.buffer.to_vec()).read_set_i32(self.offset);
        vec.into_iter().collect()
    }
}

impl<'a> FieldModel for FieldModelSetI32<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    } // Pointer only

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let pointer = u32::from_le_bytes([
            self.buffer[self.offset],
            self.buffer[self.offset + 1],
            self.buffer[self.offset + 2],
            self.buffer[self.offset + 3],
        ]) as usize;

        if pointer == 0 || self.buffer.len() < pointer + 4 {
            return 0;
        }

        let count = u32::from_le_bytes([
            self.buffer[pointer],
            self.buffer[pointer + 1],
            self.buffer[pointer + 2],
            self.buffer[pointer + 3],
        ]) as usize;

        4 + (count * 4) // size + elements
    }
}

pub struct FieldModelSetI32Mut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelSetI32Mut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, values: &HashSet<i32>) {
        let vec: Vec<i32> = values.iter().copied().collect();
        self.buffer.write_set_i32(self.offset, &vec);
    }
}

impl<'a> FieldModel for FieldModelSetI32Mut<'a> {
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
// String Collections
// ============================================================================

pub struct FieldModelVectorString<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> FieldModelVectorString<'a> {
    pub fn new(buffer: &'a [u8], offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn get(&self) -> Vec<String> {
        ReadBuffer::from(self.buffer.to_vec()).read_vector_string(self.offset)
    }
}

impl<'a> FieldModel for FieldModelVectorString<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        4
    }

    fn extra(&self) -> usize {
        if self.buffer.len() < self.offset + 4 {
            return 0;
        }
        let pointer = u32::from_le_bytes([
            self.buffer[self.offset],
            self.buffer[self.offset + 1],
            self.buffer[self.offset + 2],
            self.buffer[self.offset + 3],
        ]) as usize;

        if pointer == 0 || self.buffer.len() < pointer + 4 {
            return 0;
        }

        let size = u32::from_le_bytes([
            self.buffer[pointer],
            self.buffer[pointer + 1],
            self.buffer[pointer + 2],
            self.buffer[pointer + 3],
        ]) as usize;

        let mut total = 4;
        let mut current_offset = pointer + 4;
        for _ in 0..size {
            if self.buffer.len() < current_offset + 4 {
                break;
            }
            let len = u32::from_le_bytes([
                self.buffer[current_offset],
                self.buffer[current_offset + 1],
                self.buffer[current_offset + 2],
                self.buffer[current_offset + 3],
            ]) as usize;
            total += 4 + len;
            current_offset += 4 + len;
        }
        total
    }
}

pub struct FieldModelVectorStringMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
}

impl<'a> FieldModelVectorStringMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn set(&mut self, values: &[String]) {
        self.buffer.write_vector_string(self.offset, values);
    }
}

impl<'a> FieldModel for FieldModelVectorStringMut<'a> {
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

pub struct FieldModelArrayString<'a> {
    buffer: &'a [u8],
    offset: usize,
    count: usize,
}

impl<'a> FieldModelArrayString<'a> {
    pub fn new(buffer: &'a [u8], offset: usize, count: usize) -> Self {
        Self {
            buffer,
            offset,
            count,
        }
    }

    pub fn get(&self) -> Vec<String> {
        ReadBuffer::from(self.buffer.to_vec()).read_array_string(self.offset, self.count)
    }
}

impl<'a> FieldModel for FieldModelArrayString<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        // Variable size, calculate from buffer
        let mut total = 0;
        let mut current_offset = self.offset;
        for _ in 0..self.count {
            if self.buffer.len() < current_offset + 4 {
                break;
            }
            let len = u32::from_le_bytes([
                self.buffer[current_offset],
                self.buffer[current_offset + 1],
                self.buffer[current_offset + 2],
                self.buffer[current_offset + 3],
            ]) as usize;
            total += 4 + len;
            current_offset += 4 + len;
        }
        total
    }
}

pub struct FieldModelArrayStringMut<'a> {
    buffer: &'a mut WriteBuffer,
    offset: usize,
    count: usize,
}

impl<'a> FieldModelArrayStringMut<'a> {
    pub fn new(buffer: &'a mut WriteBuffer, offset: usize, count: usize) -> Self {
        Self {
            buffer,
            offset,
            count,
        }
    }

    pub fn set(&mut self, values: &[String]) {
        if values.len() != self.count {
            panic!(
                "Array size mismatch: expected {}, got {}",
                self.count,
                values.len()
            );
        }
        self.buffer.write_array_string(self.offset, values);
    }
}

impl<'a> FieldModel for FieldModelArrayStringMut<'a> {
    fn offset(&self) -> usize {
        self.offset
    }
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    fn size(&self) -> usize {
        0
    } // Variable
}
