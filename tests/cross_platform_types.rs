use fbe::buffer::{WriteBuffer, ReadBuffer};
use std::fs;

#[test]
fn test_cross_platform_types() {
    // Write Rust binary
    let mut writer = WriteBuffer::new();
    
    // Timestamp
    let timestamp: u64 = 1729526400000000000;
    writer.allocate(8);
    writer.write_timestamp(0, timestamp);
    
    // UUID
    let uuid: [u8; 16] = [
        0x12, 0x3e, 0x45, 0x67, 0xe8, 0x9b, 0x12, 0xd3,
        0xa4, 0x56, 0x42, 0x66, 0x55, 0x44, 0x00, 0x00,
    ];
    writer.allocate(16);
    writer.write_uuid(8, &uuid);
    
    // Bytes
    let bytes_data = b"Binary\x00\xFF";
    writer.allocate(4 + bytes_data.len());
    writer.write_bytes(24, bytes_data);
    
    // Decimal
    let decimal_value: i128 = 123456123456;
    let decimal_scale: u8 = 6;
    let decimal_negative = false;
    writer.allocate(16);
    writer.write_decimal(24 + 4 + bytes_data.len(), decimal_value, decimal_scale, decimal_negative);
    
    // Save to file
    fs::write("/tmp/rust_types.bin", writer.data()).unwrap();
    println!("Rust wrote {} bytes to /tmp/rust_types.bin", writer.size());
    
    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    assert_eq!(reader.read_timestamp(0), timestamp);
    assert_eq!(reader.read_uuid(8), uuid);
    assert_eq!(reader.read_bytes(24), bytes_data.to_vec());
    
    let (val, scale, neg) = reader.read_decimal(24 + 4 + bytes_data.len());
    assert_eq!(val, decimal_value);
    assert_eq!(scale, decimal_scale);
    assert_eq!(neg, decimal_negative);
    
    // Try reading PHP binary if exists
    if std::path::Path::new("/tmp/php_types.bin").exists() {
        println!("\nReading PHP binary...");
        let php_binary = fs::read("/tmp/php_types.bin").unwrap();
        let mut php_reader = ReadBuffer::new();
        php_reader.attach_buffer(&php_binary, 0, php_binary.len());
        
        let php_timestamp = php_reader.read_timestamp(0);
        let php_uuid = php_reader.read_uuid(8);
        let php_bytes = php_reader.read_bytes(24);
        let (php_val, php_scale, php_neg) = php_reader.read_decimal(24 + 4 + php_bytes.len());
        
        println!("PHP→Rust timestamp: {}", php_timestamp);
        println!("PHP→Rust UUID: {:02x?}", php_uuid);
        println!("PHP→Rust bytes length: {}", php_bytes.len());
        println!("PHP→Rust decimal: value={}, scale={}, negative={}", php_val, php_scale, php_neg);
        
        assert_eq!(php_timestamp, timestamp);
        assert_eq!(php_uuid, uuid);
        assert_eq!(php_bytes, bytes_data.to_vec());
        assert_eq!(php_val, decimal_value);
        assert_eq!(php_scale, decimal_scale);
        assert_eq!(php_neg, decimal_negative);
        
        println!("✓ Cross-platform types test passed!");
    }
}

