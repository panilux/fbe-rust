/*!
 * FBE Model/FinalModel Support for Rust
 */

use crate::buffer::{ReadBuffer, WriteBuffer};

// Product struct for testing
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            price: 0.0,
            quantity: 0,
        }
    }
}

impl Product {
    pub fn new(id: i32, name: String, price: f64, quantity: i32) -> Self {
        Self {
            id,
            name,
            price,
            quantity,
        }
    }

    /// Calculate struct size (without header)
    fn struct_size(&self) -> usize {
        4 + 4 + self.name.len() + 8 + 4
    }

    /// Serialize struct data (without header)
    fn serialize_struct(&self, buffer: &mut WriteBuffer, offset: usize) -> usize {
        let mut off = offset;

        buffer.write_i32(off, self.id);
        off += 4;

        buffer.write_string(off, &self.name);
        off += 4 + self.name.len();

        buffer.write_f64(off, self.price);
        off += 8;

        buffer.write_i32(off, self.quantity);
        off += 4;

        off
    }

    /// Deserialize struct data (without header)
    fn deserialize_struct(buffer: &ReadBuffer, offset: usize) -> Self {
        let mut off = offset;

        let id = buffer.read_i32(off);
        off += 4;

        let name = buffer.read_string(off);
        off += 4 + name.len();

        let price = buffer.read_f64(off);
        off += 8;

        let quantity = buffer.read_i32(off);

        Self {
            id,
            name,
            price,
            quantity,
        }
    }

    /// Serialize with Model (4-byte header)
    /// Format: [4-byte size][struct data]
    pub fn serialize_model(&self, buffer: &mut WriteBuffer) -> usize {
        let struct_size = self.struct_size();
        let total_size = 4 + struct_size; // Header + data

        buffer.reserve(total_size);

        // Write size header (little-endian u32)
        buffer.write_u32(0, total_size as u32);

        // Write struct data
        self.serialize_struct(buffer, 4);

        buffer.set_size(total_size);
        total_size
    }

    /// Deserialize with Model (4-byte header)
    pub fn deserialize_model(buffer: &ReadBuffer) -> (Self, usize) {
        // Read size header
        let total_size = buffer.read_u32(0) as usize;

        // Deserialize struct data
        let product = Self::deserialize_struct(buffer, 4);

        (product, total_size)
    }

    /// Serialize with FinalModel (no header)
    /// Format: [struct data]
    pub fn serialize_final(&self, buffer: &mut WriteBuffer) -> usize {
        let struct_size = self.struct_size();

        buffer.reserve(struct_size);

        // Write struct data directly
        let end = self.serialize_struct(buffer, 0);

        buffer.set_size(end);
        end
    }

    /// Deserialize with FinalModel (no header)
    pub fn deserialize_final(buffer: &ReadBuffer) -> (Self, usize) {
        let product = Self::deserialize_struct(buffer, 0);
        let size = buffer.data().len();

        (product, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_with_header() {
        let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

        let mut buffer = WriteBuffer::new();
        let model_size = product.serialize_model(&mut buffer);

        println!("Model size: {} bytes", model_size);
        println!("Model data (hex): {}", hex::encode(buffer.data()));

        // Verify header
        let mut read_buf = ReadBuffer::new();
        read_buf.attach_buffer(buffer.data(), 0, buffer.data().len());
        let header = read_buf.read_u32(0);
        assert_eq!(header, model_size as u32);

        // Deserialize
        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let (product2, read_size) = Product::deserialize_model(&read_buffer);

        assert_eq!(product2.id, 123);
        assert_eq!(product2.name, "Laptop");
        assert_eq!(product2.price, 999.99);
        assert_eq!(product2.quantity, 5);
        assert_eq!(read_size, model_size);
    }

    #[test]
    fn test_final_model_without_header() {
        let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

        let mut buffer = WriteBuffer::new();
        let final_size = product.serialize_final(&mut buffer);

        println!("FinalModel size: {} bytes", final_size);
        println!("FinalModel data (hex): {}", hex::encode(buffer.data()));

        // Deserialize
        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let (product2, read_size) = Product::deserialize_final(&read_buffer);

        assert_eq!(product2.id, 123);
        assert_eq!(product2.name, "Laptop");
        assert_eq!(product2.price, 999.99);
        assert_eq!(product2.quantity, 5);
        assert_eq!(read_size, final_size);
    }

    #[test]
    fn test_size_comparison() {
        let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

        // Model
        let mut model_buffer = WriteBuffer::new();
        let model_size = product.serialize_model(&mut model_buffer);

        // FinalModel
        let mut final_buffer = WriteBuffer::new();
        let final_size = product.serialize_final(&mut final_buffer);

        println!("Model size: {} bytes (with header)", model_size);
        println!("FinalModel size: {} bytes (no header)", final_size);
        println!("Difference: {} bytes", model_size - final_size);

        // Model should be 4 bytes larger (header)
        assert_eq!(model_size, final_size + 4);
    }

    #[test]
    fn test_data_comparison() {
        let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

        // Model
        let mut model_buffer = WriteBuffer::new();
        product.serialize_model(&mut model_buffer);

        // FinalModel
        let mut final_buffer = WriteBuffer::new();
        product.serialize_final(&mut final_buffer);

        // Skip 4-byte header in Model
        let model_data_without_header = &model_buffer.data()[4..];
        let final_data = final_buffer.data();

        println!(
            "Model data (without header): {}",
            hex::encode(model_data_without_header)
        );
        println!("FinalModel data:             {}", hex::encode(final_data));

        // Data should be identical (excluding header)
        assert_eq!(model_data_without_header, final_data);
    }
}
