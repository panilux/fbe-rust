use fbe::buffer::{WriteBuffer, ReadBuffer};

#[test]
fn test_timestamp() {
    let mut writer = WriteBuffer::with_capacity(100);
    writer.allocate(8);
    
    let timestamp: u64 = 1729526400000000000; // 2024-10-21 12:00:00 UTC in nanoseconds
    writer.write_timestamp(0, timestamp);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_timestamp = reader.read_timestamp(0);
    assert_eq!(timestamp, read_timestamp);
}

#[test]
fn test_uuid() {
    let mut writer = WriteBuffer::with_capacity(100);
    writer.allocate(16);
    
    let uuid: [u8; 16] = [
        0x12, 0x3e, 0x45, 0x67, 0xe8, 0x9b, 0x12, 0xd3,
        0xa4, 0x56, 0x42, 0x66, 0x55, 0x44, 0x00, 0x00,
    ];
    writer.write_uuid(0, &uuid);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_uuid = reader.read_uuid(0);
    assert_eq!(uuid, read_uuid);
}

#[test]
fn test_bytes() {
    let mut writer = WriteBuffer::with_capacity(100);
    let data = b"Binary data test \x00\xFF\xAB";
    writer.allocate(4 + data.len());
    
    writer.write_bytes(0, data);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_bytes = reader.read_bytes(0);
    assert_eq!(data.to_vec(), read_bytes);
}

#[test]
fn test_decimal() {
    let mut writer = WriteBuffer::with_capacity(100);
    writer.allocate(16);
    
    // Test: 123456.123456 with scale 6
    let value: i128 = 123456123456;
    let scale: u8 = 6;
    let negative = false;
    
    writer.write_decimal(0, value, scale, negative);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let (read_value, read_scale, read_negative) = reader.read_decimal(0);
    assert_eq!(value, read_value);
    assert_eq!(scale, read_scale);
    assert_eq!(negative, read_negative);
}

#[test]
fn test_decimal_negative() {
    let mut writer = WriteBuffer::with_capacity(100);
    writer.allocate(16);
    
    // Test: -999.99 with scale 2
    let value: i128 = 99999;
    let scale: u8 = 2;
    let negative = true;
    
    writer.write_decimal(0, value, scale, negative);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let (read_value, read_scale, read_negative) = reader.read_decimal(0);
    assert_eq!(value, read_value);
    assert_eq!(scale, read_scale);
    assert_eq!(negative, read_negative);
}

