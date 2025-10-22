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

    /// Set buffer size
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
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
        assert!(offset + size <= self.buffer.len(), "Invalid offset & size!");

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
    pub fn write_byte(&mut self, offset: usize, value: u8) {
        self.write_u8(offset, value);
    }

    #[inline]
    pub fn write_char(&mut self, offset: usize, value: u8) {
        self.buffer[self.offset + offset] = value;
    }

    #[inline]
    pub fn write_wchar(&mut self, offset: usize, value: u32) {
        self.write_u32(offset, value);
    }

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
        self.buffer[self.offset + offset + 4..self.offset + offset + 4 + bytes.len()]
            .copy_from_slice(bytes);
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
        self.buffer[self.offset + offset + 4..self.offset + offset + 4 + value.len()]
            .copy_from_slice(value);
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

    /// Write list of i32 values (linked list, same format as vector)
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    pub fn write_list_i32(&mut self, offset: usize, values: &[i32]) -> usize {
        // List uses same format as vector (pointer-based)
        self.write_vector_i32(offset, values)
    }

    /// Write vector of i32 values
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    pub fn write_vector_i32(&mut self, offset: usize, values: &[i32]) -> usize {
        let size = values.len();
        let data_size = 4 + (size * 4); // 4 bytes size + elements
        let data_offset = self.allocate(data_size);

        // Write pointer at offset
        self.write_u32(offset, (data_offset - self.offset) as u32);

        // Write size at data_offset
        self.write_u32(data_offset - self.offset, size as u32);

        // Write elements
        for (i, &value) in values.iter().enumerate() {
            self.write_i32(data_offset - self.offset + 4 + (i * 4), value);
        }

        data_size
    }

    /// Write fixed-size array of i32 values (inline, no pointer)
    /// Format: N × 4 bytes (elements only)
    #[inline]
    pub fn write_array_i32(&mut self, offset: usize, values: &[i32]) {
        for (i, &value) in values.iter().enumerate() {
            self.write_i32(offset + (i * 4), value);
        }
    }

    /// Write map of i32 key-value pairs
    /// Format: 4-byte offset pointer → (4-byte size + key-value pairs)
    pub fn write_map_i32(&mut self, offset: usize, entries: &[(i32, i32)]) -> usize {
        let size = entries.len();
        let data_size = 4 + (size * 8); // 4 bytes size + (key+value) pairs
        let data_offset = self.allocate(data_size);

        // Write pointer at offset
        self.write_u32(offset, (data_offset - self.offset) as u32);

        // Write size at data_offset
        self.write_u32(data_offset - self.offset, size as u32);

        // Write key-value pairs
        for (i, &(key, value)) in entries.iter().enumerate() {
            self.write_i32(data_offset - self.offset + 4 + (i * 8), key);
            self.write_i32(data_offset - self.offset + 4 + (i * 8) + 4, value);
        }

        data_size
    }

    /// Write set of i32 values (unique values, same format as vector)
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    /// Note: Uniqueness constraint enforced at application level
    #[inline]
    pub fn write_set_i32(&mut self, offset: usize, values: &[i32]) -> usize {
        self.write_vector_i32(offset, values)
    }
    // ========================================================================
    // String Collections
    // ========================================================================

    /// Write vector of strings
    pub fn write_vector_string(&mut self, offset: usize, values: &[String]) -> usize {
        let size = values.len();
        let mut data_size = 4; // size prefix
        for s in values {
            data_size += 4 + s.len();
        }

        let data_offset = self.allocate(data_size);
        self.write_u32(offset, (data_offset - self.offset) as u32);
        self.write_u32(data_offset - self.offset, size as u32);

        let mut current_offset = data_offset - self.offset + 4;
        for s in values {
            self.write_string(current_offset, s);
            current_offset += 4 + s.len();
        }

        data_size
    }

    /// Write fixed-size array of strings
    pub fn write_array_string(&mut self, offset: usize, values: &[String]) -> usize {
        let mut current_offset = offset;
        for s in values {
            self.write_string(current_offset, s);
            current_offset += 4 + s.len();
        }
        current_offset - offset
    }

    // ========================================================================
    // Float/Double Collections
    // ========================================================================

    pub fn write_vector_f32(&mut self, offset: usize, values: &[f32]) -> usize {
        let size = values.len();
        let data_size = 4 + (size * 4);
        let data_offset = self.allocate(data_size);
        self.write_u32(offset, (data_offset - self.offset) as u32);
        self.write_u32(data_offset - self.offset, size as u32);
        for (i, &value) in values.iter().enumerate() {
            self.write_f32(data_offset - self.offset + 4 + (i * 4), value);
        }
        data_size
    }

    pub fn write_array_f32(&mut self, offset: usize, values: &[f32]) -> usize {
        for (i, &value) in values.iter().enumerate() {
            self.write_f32(offset + (i * 4), value);
        }
        values.len() * 4
    }

    pub fn write_vector_f64(&mut self, offset: usize, values: &[f64]) -> usize {
        let size = values.len();
        let data_size = 4 + (size * 8);
        let data_offset = self.allocate(data_size);
        self.write_u32(offset, (data_offset - self.offset) as u32);
        self.write_u32(data_offset - self.offset, size as u32);
        for (i, &value) in values.iter().enumerate() {
            self.write_f64(data_offset - self.offset + 4 + (i * 8), value);
        }
        data_size
    }

    pub fn write_array_f64(&mut self, offset: usize, values: &[f64]) -> usize {
        for (i, &value) in values.iter().enumerate() {
            self.write_f64(offset + (i * 8), value);
        }
        values.len() * 8
    }

    // Optional types
    pub fn write_optional_i32(&mut self, offset: usize, value: Option<i32>) {
        match value {
            None => self.write_u8(offset, 0),
            Some(v) => {
                self.write_u8(offset, 1);
                let data_offset = self.allocate(4); // Allocate for i32
                self.write_u32(offset + 1, data_offset as u32);
                self.write_i32(data_offset, v);
            }
        }
    }

    pub fn write_optional_string(&mut self, offset: usize, value: Option<&str>) {
        match value {
            None => self.write_u8(offset, 0),
            Some(v) => {
                self.write_u8(offset, 1);
                let len = v.len();
                let data_offset = self.allocate(4 + len); // Allocate for length + string
                self.write_u32(offset + 1, data_offset as u32);
                self.write_string(data_offset, v);
            }
        }
    }

    pub fn write_optional_f64(&mut self, offset: usize, value: Option<f64>) {
        match value {
            None => self.write_u8(offset, 0),
            Some(v) => {
                self.write_u8(offset, 1);
                let data_offset = self.allocate(8); // Allocate for f64
                self.write_u32(offset + 1, data_offset as u32);
                self.write_f64(data_offset, v);
            }
        }
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

impl From<Vec<u8>> for ReadBuffer {
    fn from(data: Vec<u8>) -> Self {
        let size = data.len();
        Self {
            buffer: data,
            size,
            offset: 0,
        }
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
    pub fn read_byte(&self, offset: usize) -> u8 {
        self.read_u8(offset)
    }

    #[must_use]
    #[inline]
    pub fn read_char(&self, offset: usize) -> u8 {
        self.buffer[self.offset + offset]
    }

    #[must_use]
    #[inline]
    pub fn read_wchar(&self, offset: usize) -> u32 {
        self.read_u32(offset)
    }

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
        value_bytes[..12]
            .copy_from_slice(&self.buffer[self.offset + offset..self.offset + offset + 12]);
        let value = i128::from_le_bytes(value_bytes);

        // Read scale from byte 14
        let scale = self.buffer[self.offset + offset + 14];

        // Read sign from byte 15
        let negative = (self.buffer[self.offset + offset + 15] & 0x80) != 0;

        (value, scale, negative)
    }

    /// Read list of i32 values (linked list, same format as vector)
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    #[must_use]
    pub fn read_list_i32(&self, offset: usize) -> Vec<i32> {
        // List uses same format as vector (pointer-based)
        self.read_vector_i32(offset)
    }

    /// Read vector of i32 values
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    #[must_use]
    pub fn read_vector_i32(&self, offset: usize) -> Vec<i32> {
        // Read pointer
        let data_offset = self.read_u32(offset) as usize;
        if data_offset == 0 {
            return Vec::new();
        }

        // Read size
        let size = self.read_u32(data_offset) as usize;

        // Read elements
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.read_i32(data_offset + 4 + (i * 4)));
        }

        result
    }

    /// Read fixed-size array of i32 values (inline, no pointer)
    /// Format: N × 4 bytes (elements only)
    #[must_use]
    #[inline]
    pub fn read_array_i32(&self, offset: usize, size: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.read_i32(offset + (i * 4)));
        }
        result
    }

    /// Read map of i32 key-value pairs
    /// Format: 4-byte offset pointer → (4-byte size + key-value pairs)
    #[must_use]
    pub fn read_map_i32(&self, offset: usize) -> Vec<(i32, i32)> {
        // Read pointer
        let data_offset = self.read_u32(offset) as usize;
        if data_offset == 0 {
            return Vec::new();
        }

        // Read size
        let size = self.read_u32(data_offset) as usize;

        // Read key-value pairs
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            let key = self.read_i32(data_offset + 4 + (i * 8));
            let value = self.read_i32(data_offset + 4 + (i * 8) + 4);
            result.push((key, value));
        }

        result
    }

    /// Read set of i32 values (same format as vector)
    /// Format: 4-byte offset pointer → (4-byte size + elements)
    #[must_use]
    #[inline]
    pub fn read_set_i32(&self, offset: usize) -> Vec<i32> {
        self.read_vector_i32(offset)
    }

    // ========================================================================
    // String Collections
    // ========================================================================

    /// Read vector of strings
    pub fn read_vector_string(&self, offset: usize) -> Vec<String> {
        let pointer = self.read_u32(offset) as usize;
        if pointer == 0 {
            return Vec::new();
        }

        let size = self.read_u32(pointer) as usize;
        let mut values = Vec::with_capacity(size);
        let mut current_offset = pointer + 4;

        for _ in 0..size {
            let s = self.read_string(current_offset);
            current_offset += 4 + s.len();
            values.push(s);
        }

        values
    }

    /// Read fixed-size array of strings
    pub fn read_array_string(&self, offset: usize, count: usize) -> Vec<String> {
        let mut values = Vec::with_capacity(count);
        let mut current_offset = offset;

        for _ in 0..count {
            let s = self.read_string(current_offset);
            current_offset += 4 + s.len();
            values.push(s);
        }

        values
    }

    // ========================================================================
    // Float/Double Collections
    // ========================================================================

    pub fn read_vector_f32(&self, offset: usize) -> Vec<f32> {
        let pointer = self.read_u32(offset) as usize;
        if pointer == 0 {
            return Vec::new();
        }
        let size = self.read_u32(pointer) as usize;
        let mut values = Vec::with_capacity(size);
        for i in 0..size {
            values.push(self.read_f32(pointer + 4 + (i * 4)));
        }
        values
    }

    pub fn read_array_f32(&self, offset: usize, count: usize) -> Vec<f32> {
        let mut values = Vec::with_capacity(count);
        for i in 0..count {
            values.push(self.read_f32(offset + (i * 4)));
        }
        values
    }

    pub fn read_vector_f64(&self, offset: usize) -> Vec<f64> {
        let pointer = self.read_u32(offset) as usize;
        if pointer == 0 {
            return Vec::new();
        }
        let size = self.read_u32(pointer) as usize;
        let mut values = Vec::with_capacity(size);
        for i in 0..size {
            values.push(self.read_f64(pointer + 4 + (i * 8)));
        }
        values
    }

    pub fn read_array_f64(&self, offset: usize, count: usize) -> Vec<f64> {
        let mut values = Vec::with_capacity(count);
        for i in 0..count {
            values.push(self.read_f64(offset + (i * 8)));
        }
        values
    }

    // Optional types
    pub fn has_value(&self, offset: usize) -> bool {
        self.read_u8(offset) != 0
    }

    pub fn read_optional_i32(&self, offset: usize) -> Option<i32> {
        if !self.has_value(offset) {
            return None;
        }
        let data_offset = self.read_u32(offset + 1) as usize;
        Some(self.read_i32(data_offset))
    }

    pub fn read_optional_string(&self, offset: usize) -> Option<String> {
        if !self.has_value(offset) {
            return None;
        }
        let data_offset = self.read_u32(offset + 1) as usize;
        Some(self.read_string(data_offset))
    }

    pub fn read_optional_f64(&self, offset: usize) -> Option<f64> {
        if !self.has_value(offset) {
            return None;
        }
        let data_offset = self.read_u32(offset + 1) as usize;
        Some(self.read_f64(data_offset))
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
