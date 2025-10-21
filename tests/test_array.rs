use fbe::buffer::{WriteBuffer, ReadBuffer};

#[test]
fn test_array_i32() {
    let mut writer = WriteBuffer::new();
    
    // Allocate space for 3 int32 values (12 bytes)
    writer.allocate(12);
    
    // Write array
    let values = [10, 20, 30];
    writer.write_array_i32(0, &values);
    
    println!("Buffer size: {}", writer.size());
    println!("Binary: {:02x?}", writer.data());
    
    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_array_i32(0, 3);
    
    assert_eq!(values.to_vec(), read_values);
    println!("✓ Array test passed: {:?}", read_values);
}

#[test]
fn test_array_large() {
    let mut writer = WriteBuffer::new();
    
    // Allocate space for 100 int32 values (400 bytes)
    writer.allocate(400);
    
    let values: Vec<i32> = (0..100).collect();
    writer.write_array_i32(0, &values);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_array_i32(0, 100);
    
    assert_eq!(values, read_values);
    println!("✓ Large array test passed (100 elements)");
}

