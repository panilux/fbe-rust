//! FBE Order struct test - 100% FBE C++ proto spec compliant
//!
//! Based on proto.fbe:
//! ```
//! enum OrderSide : byte {
//!     buy;    // 0
//!     sell;   // 1
//! }
//!
//! enum OrderType : byte {
//!     market; // 0
//!     limit;  // 1
//!     stop;   // 2
//! }
//!
//! struct Order(1) {
//!     [key] int32 id;
//!     string symbol;
//!     OrderSide side;
//!     OrderType type;
//!     double price = 0.0;
//!     double volume = 0.0;
//! }
//! ```

use fbe::buffer::{ReadBuffer, WriteBuffer};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderSide {
    Buy = 0,
    Sell = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Market = 0,
    Limit = 1,
    Stop = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: i32,
    pub symbol: String,
    pub side: OrderSide,
    pub type_: OrderType,
    pub price: f64,
    pub volume: f64,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            id: 0,
            symbol: String::new(),
            side: OrderSide::Buy,
            type_: OrderType::Market,
            price: 0.0,
            volume: 0.0,
        }
    }
}

impl Order {
    /// Serialize Order in Standard format (with 8-byte header)
    ///
    /// Binary layout:
    /// ```
    /// [0-3]:   Struct size (4 bytes, little-endian)
    /// [4-7]:   Struct type ID = 1 (4 bytes, little-endian)
    /// [8-11]:  id (4 bytes, little-endian)
    /// [12-15]: symbol pointer (4 bytes, little-endian, relative offset)
    /// [16]:    side (1 byte)
    /// [17-19]: padding (3 bytes) ← C++ struct alignment!
    /// [20]:    type (1 byte)
    /// [21-23]: padding (3 bytes) ← C++ struct alignment!
    /// [24-31]: price (8 bytes, little-endian IEEE 754 double)
    /// [32-39]: volume (8 bytes, little-endian IEEE 754 double)
    /// Total struct size: 32 bytes (without header)
    /// ```
    pub fn serialize_standard(&self, buffer: &mut WriteBuffer) -> usize {
        const STRUCT_SIZE: u32 = 32; // Fixed struct size (without string data)
        const TYPE_ID: u32 = 1;

        // Allocate space for header + struct
        buffer.allocate(8 + STRUCT_SIZE as usize);

        // Write 8-byte header
        buffer.write_u32(0, STRUCT_SIZE); // Struct size
        buffer.write_u32(4, TYPE_ID);     // Type ID

        // Write Order fields (starting at offset 8)
        buffer.write_i32(8, self.id);

        // Symbol: write pointer (we'll write string data later)
        let symbol_len = self.symbol.len();
        let string_offset = buffer.allocate(4 + symbol_len);
        buffer.write_u32(12, (string_offset - buffer.offset()) as u32); // Relative pointer
        buffer.write_string(string_offset - buffer.offset(), &self.symbol);

        // Side enum (1 byte)
        buffer.write_u8(16, self.side as u8);

        // Padding (3 bytes) - C++ struct alignment
        buffer.write_u8(17, 0);
        buffer.write_u8(18, 0);
        buffer.write_u8(19, 0);

        // Type enum (1 byte)
        buffer.write_u8(20, self.type_ as u8);

        // Padding (3 bytes) - C++ struct alignment
        buffer.write_u8(21, 0);
        buffer.write_u8(22, 0);
        buffer.write_u8(23, 0);

        // Price and Volume (doubles)
        buffer.write_f64(24, self.price);
        buffer.write_f64(32, self.volume);

        buffer.size()
    }

    /// Deserialize Order from Standard format
    pub fn deserialize_standard(buffer: &ReadBuffer) -> Self {
        // Read header
        let _struct_size = buffer.read_u32(0);
        let _type_id = buffer.read_u32(4);

        // Read fields
        let id = buffer.read_i32(8);

        // Symbol: follow pointer
        let symbol_ptr = buffer.read_u32(12) as usize;
        let symbol = buffer.read_string(symbol_ptr);

        // Side enum (skip padding)
        let side = match buffer.read_u8(16) {
            0 => OrderSide::Buy,
            1 => OrderSide::Sell,
            _ => OrderSide::Buy,
        };

        // Type enum (skip padding)
        let type_ = match buffer.read_u8(20) {
            0 => OrderType::Market,
            1 => OrderType::Limit,
            2 => OrderType::Stop,
            _ => OrderType::Market,
        };

        // Price and Volume
        let price = buffer.read_f64(24);
        let volume = buffer.read_f64(32);

        Order {
            id,
            symbol,
            side,
            type_,
            price,
            volume,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_standard_format_round_trip() {
        let order = Order {
            id: 12345,
            symbol: "AAPL".to_string(),
            side: OrderSide::Buy,
            type_: OrderType::Limit,
            price: 150.75,
            volume: 100.0,
        };

        let mut buffer = WriteBuffer::new();
        buffer.reserve(1024);

        let size = order.serialize_standard(&mut buffer);
        println!("Serialized size: {} bytes", size);

        // Print hex dump (first 64 bytes)
        let data = buffer.data();
        println!("\nHex dump (first 64 bytes):");
        for (i, chunk) in data.chunks(16).take(4).enumerate() {
            print!("{:04x}  ", i * 16);
            for byte in chunk {
                print!("{:02x} ", byte);
            }
            println!();
        }

        // Deserialize
        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.size());

        let deserialized = Order::deserialize_standard(&read_buffer);

        // Verify
        assert_eq!(deserialized.id, order.id);
        assert_eq!(deserialized.symbol, order.symbol);
        assert_eq!(deserialized.side, order.side);
        assert_eq!(deserialized.type_, order.type_);
        assert_eq!(deserialized.price, order.price);
        assert_eq!(deserialized.volume, order.volume);

        println!("\n✅ Order Standard Format round-trip test PASSED");
    }

    #[test]
    fn test_order_binary_format_verification() {
        let order = Order {
            id: 12345,
            symbol: "AAPL".to_string(),
            side: OrderSide::Buy,
            type_: OrderType::Limit,
            price: 150.75,
            volume: 100.0,
        };

        let mut buffer = WriteBuffer::new();
        buffer.reserve(1024);
        order.serialize_standard(&mut buffer);

        let data = buffer.data();

        // Verify header
        assert_eq!(data[0..4], [32, 0, 0, 0], "Struct size = 32");
        assert_eq!(data[4..8], [1, 0, 0, 0], "Type ID = 1");

        // Verify id = 12345 (0x3039)
        assert_eq!(data[8..12], [0x39, 0x30, 0x00, 0x00], "id = 12345");

        // Verify side = 0 (Buy)
        assert_eq!(data[16], 0, "side = Buy (0)");

        // Verify padding after side
        assert_eq!(data[17], 0, "padding byte 1");
        assert_eq!(data[18], 0, "padding byte 2");
        assert_eq!(data[19], 0, "padding byte 3");

        // Verify type = 1 (Limit)
        assert_eq!(data[20], 1, "type = Limit (1)");

        // Verify padding after type
        assert_eq!(data[21], 0, "padding byte 1");
        assert_eq!(data[22], 0, "padding byte 2");
        assert_eq!(data[23], 0, "padding byte 3");

        // Verify price = 150.75 (IEEE 754 double)
        let price_bytes = 150.75_f64.to_le_bytes();
        assert_eq!(data[24..32], price_bytes, "price = 150.75");

        // Verify volume = 100.0 (IEEE 754 double)
        let volume_bytes = 100.0_f64.to_le_bytes();
        assert_eq!(data[32..40], volume_bytes, "volume = 100.0");

        println!("\n✅ Order binary format verification PASSED");
        println!("   Header: [size: 32, type: 1]");
        println!("   Fields: id={}, symbol={}, side={:?}, type={:?}, price={}, volume={}",
                 order.id, order.symbol, order.side, order.type_, order.price, order.volume);
        println!("   Padding: Correctly added after enums (C++ struct alignment)");
    }
}
