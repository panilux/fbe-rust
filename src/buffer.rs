//! Fast Binary Encoding buffer implementation
//! 
//! Based on original FBE Python implementation with exact API compatibility

/// Write buffer for FBE serialization
/// 
/// Manages dynamic byte buffer with offset tracking and allocation
#[derive(Debug, Clone)]
pub struct WriteBuffer {
    buffer: Vec<u8>,
    size: usize,
    offset: usize,
}

impl Default for WriteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteBuffer {
    /// Create a new write buffer
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            size: 0,
            offset: 0,
        }
    }

    /// Create a new write buffer with capacity
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            size: 0,
            offset: 0,
        }
    }

    /// Check if buffer is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Get buffer data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.buffer[..self.size]
    }

    /// Get buffer capacity
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Get buffer size
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Get current offset
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Attach an empty memory buffer
    pub fn attach_new(&mut self) {
        self.buffer = Vec::new();
        self.size = 0;
        self.offset = 0;
    }

    /// Attach an empty memory buffer with capacity
    pub fn attach_capacity(&mut self, capacity: usize) {
        self.buffer = vec![0; capacity];
        self.size = 0;
        self.offset = 0;
    }

    /// Attach a given memory buffer
    pub fn attach_buffer(&mut self, buffer: &[u8], offset: usize, size: usize) {
        assert!(!buffer.is_empty(), "Invalid buffer!");
        assert!(size > 0, "Invalid size!");
        assert!(offset <= size, "Invalid offset!");

        self.buffer = buffer.to_vec();
        self.size = size;
        self.offset = offset;
    }

    /// Allocate memory and return offset to allocated block
    pub fn allocate(&mut self, size: usize) -> usize {
        let offset = self.size;
        let total = self.size + size;

        if total > self.buffer.len() {
            let new_capacity = total.max(self.buffer.len() * 2);
            self.buffer.resize(new_capacity, 0);
        }

        self.size = total;
        offset
    }

    /// Remove memory of given size from buffer
    pub fn remove(&mut self, offset: usize, size: usize) {
        assert!(
            offset + size <= self.buffer.len(),
            "Invalid offset & size!"
        );

        self.buffer.drain(offset..offset + size);
        self.size -= size;

        if self.offset >= offset + size {
            self.offset -= size;
        } else if self.offset >= offset {
            self.offset -= self.offset - offset;
            if self.offset > self.size {
                self.offset = self.size;
            }
        }
    }

    /// Reserve memory of given capacity
    pub fn reserve(&mut self, capacity: usize) {
        if capacity > self.buffer.len() {
            let new_capacity = capacity.max(self.buffer.len() * 2);
            self.buffer.resize(new_capacity, 0);
        }
    }

    /// Resize the buffer
    pub fn resize(&mut self, size: usize) {
        self.reserve(size);
        self.size = size;
        if self.offset > self.size {
            self.offset = self.size;
        }
    }

    /// Reset buffer and offset
    pub fn reset(&mut self) {
        self.size = 0;
        self.offset = 0;
    }

    /// Shift offset forward
    pub fn shift(&mut self, offset: usize) {
        self.offset += offset;
    }

    /// Shift offset backward
    pub fn unshift(&mut self, offset: usize) {
        self.offset -= offset;
    }

    // Write primitive types
    #[inline]
    pub fn write_bool(&mut self, offset: usize, value: bool) {
        self.buffer[self.offset + offset] = value as u8;
    }

    #[inline]
    pub fn write_i8(&mut self, offset: usize, value: i8) {
        self.buffer[self.offset + offset] = value as u8;
    }

    #[inline]
    pub fn write_u8(&mut self, offset: usize, value: u8) {
        self.buffer[self.offset + offset] = value;
    }

    #[inline]
    pub fn write_i16(&mut self, offset: usize, value: i16) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 2].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_u16(&mut self, offset: usize, value: u16) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 2].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_i32(&mut self, offset: usize, value: i32) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 4].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_u32(&mut self, offset: usize, value: u32) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 4].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_i64(&mut self, offset: usize, value: i64) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 8].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_u64(&mut self, offset: usize, value: u64) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 8].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_f32(&mut self, offset: usize, value: f32) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 4].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_f64(&mut self, offset: usize, value: f64) {
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 8].copy_from_slice(&bytes);
    }

    #[inline]
    pub fn write_string(&mut self, offset: usize, value: &str) {
        let len = value.len() as i32;
        self.write_i32(offset, len);
        let bytes = value.as_bytes();
        self.buffer[self.offset + offset + 4..self.offset + offset + 4 + bytes.len()].copy_from_slice(bytes);
    }

    /// Write timestamp as uint64 (nanoseconds since epoch)
    #[inline]
    pub fn write_timestamp(&mut self, offset: usize, value: u64) {
        self.write_u64(offset, value);
    }

    /// Write UUID as 16 bytes (big-endian format)
    #[inline]
    pub fn write_uuid(&mut self, offset: usize, value: &[u8; 16]) {
        self.buffer[self.offset + offset..self.offset + offset + 16].copy_from_slice(value);
    }

    /// Write bytes (size-prefixed binary data)
    #[inline]
    pub fn write_bytes(&mut self, offset: usize, value: &[u8]) {
        let len = value.len() as i32;
        self.write_i32(offset, len);
        self.buffer[self.offset + offset + 4..self.offset + offset + 4 + value.len()].copy_from_slice(value);
    }

    /// Write decimal as 16 bytes (.NET Decimal format)
    /// Format: bytes 0-11 = unscaled value (96-bit), byte 14 = scale, byte 15 = sign
    #[inline]
    pub fn write_decimal(&mut self, offset: usize, value: i128, scale: u8, negative: bool) {
        // Write unscaled value to bytes 0-11 (96-bit little-endian)
        let bytes = value.to_le_bytes();
        self.buffer[self.offset + offset..self.offset + offset + 12].copy_from_slice(&bytes[..12]);
        // Bytes 12-13 are unused (zero)
        self.buffer[self.offset + offset + 12] = 0;
        self.buffer[self.offset + offset + 13] = 0;
        // Byte 14 = scale
        self.buffer[self.offset + offset + 14] = scale;
        // Byte 15 = sign
        self.buffer[self.offset + offset + 15] = if negative { 0x80 } else { 0x00 };
    }
}

/// Read buffer for FBE deserialization
#[derive(Debug, Clone)]
pub struct ReadBuffer {
    buffer: Vec<u8>,
    size: usize,
    offset: usize,
}

impl Default for ReadBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadBuffer {
    /// Create a new read buffer
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            size: 0,
            offset: 0,
        }
    }

    /// Get buffer data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.buffer
    }

    /// Get buffer capacity
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.size
    }

    /// Get buffer size
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Get current offset
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Attach a given memory buffer
    pub fn attach_buffer(&mut self, buffer: &[u8], offset: usize, size: usize) {
        assert!(!buffer.is_empty(), "Invalid buffer!");
        assert!(size > 0, "Invalid size!");
        assert!(offset <= size, "Invalid offset!");

        self.buffer = buffer.to_vec();
        self.size = size;
        self.offset = offset;
    }

    /// Reset buffer and offset
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.size = 0;
        self.offset = 0;
    }

    /// Shift offset forward
    pub fn shift(&mut self, offset: usize) {
        self.offset += offset;
    }

    /// Shift offset backward
    pub fn unshift(&mut self, offset: usize) {
        self.offset -= offset;
    }

    // Read primitive types
    #[must_use]
    #[inline]
    pub fn read_bool(&self, offset: usize) -> bool {
        self.buffer[self.offset + offset] != 0
    }

    #[must_use]
    #[inline]
    pub fn read_i8(&self, offset: usize) -> i8 {
        self.buffer[self.offset + offset] as i8
    }

    #[must_use]
    #[inline]
    pub fn read_u8(&self, offset: usize) -> u8 {
        self.buffer[self.offset + offset]
    }

    #[must_use]
    #[inline]
    pub fn read_i16(&self, offset: usize) -> i16 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 2];
        i16::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_u16(&self, offset: usize) -> u16 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 2];
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_i32(&self, offset: usize) -> i32 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 4];
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_u32(&self, offset: usize) -> u32 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 4];
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_i64(&self, offset: usize) -> i64 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 8];
        i64::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_u64(&self, offset: usize) -> u64 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 8];
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_f32(&self, offset: usize) -> f32 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 4];
        f32::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_f64(&self, offset: usize) -> f64 {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 8];
        f64::from_le_bytes(bytes.try_into().unwrap())
    }

    #[must_use]
    #[inline]
    pub fn read_string(&self, offset: usize) -> String {
        let len = self.read_i32(offset) as usize;
        let bytes = &self.buffer[self.offset + offset + 4..self.offset + offset + 4 + len];
        String::from_utf8_lossy(bytes).to_string()
    }

    /// Read timestamp as uint64 (nanoseconds since epoch)
    #[must_use]
    #[inline]
    pub fn read_timestamp(&self, offset: usize) -> u64 {
        self.read_u64(offset)
    }

    /// Read UUID as 16 bytes
    #[must_use]
    #[inline]
    pub fn read_uuid(&self, offset: usize) -> [u8; 16] {
        let bytes = &self.buffer[self.offset + offset..self.offset + offset + 16];
        bytes.try_into().unwrap()
    }

    /// Read bytes (size-prefixed binary data)
    #[must_use]
    #[inline]
    pub fn read_bytes(&self, offset: usize) -> Vec<u8> {
        let len = self.read_i32(offset) as usize;
        self.buffer[self.offset + offset + 4..self.offset + offset + 4 + len].to_vec()
    }

    /// Read decimal as (value, scale, negative)
    /// Returns: (unscaled i128 value, scale u8, is_negative bool)
    #[must_use]
    #[inline]
    pub fn read_decimal(&self, offset: usize) -> (i128, u8, bool) {
        // Read 96-bit unscaled value from bytes 0-11
        let mut value_bytes = [0u8; 16];
        value_bytes[..12].copy_from_slice(&self.buffer[self.offset + offset..self.offset + offset + 12]);
        let value = i128::from_le_bytes(value_bytes);
        
        // Read scale from byte 14
        let scale = self.buffer[self.offset + offset + 14];
        
        // Read sign from byte 15
        let negative = (self.buffer[self.offset + offset + 15] & 0x80) != 0;
        
        (value, scale, negative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_buffer_basic() {
        let mut buffer = WriteBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.size(), 0);
        assert_eq!(buffer.offset(), 0);
    }

    #[test]
    fn test_write_buffer_allocate() {
        let mut buffer = WriteBuffer::new();
        let offset = buffer.allocate(100);
        assert_eq!(offset, 0);
        assert_eq!(buffer.size(), 100);
    }

    #[test]
    fn test_write_read_primitives() {
        let mut writer = WriteBuffer::with_capacity(100);
        writer.allocate(100);
        
        writer.write_i32(0, 42);
        writer.write_f64(4, 3.14159);
        writer.write_bool(12, true);

        let mut reader = ReadBuffer::new();
        reader.attach_buffer(writer.data(), 0, writer.size());

        assert_eq!(reader.read_i32(0), 42);
        assert_eq!(reader.read_f64(4), 3.14159);
        assert_eq!(reader.read_bool(12), true);
    }
}

