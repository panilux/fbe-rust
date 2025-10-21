use fbe::buffer::{WriteBuffer, ReadBuffer};

#[test]
fn test_vector_i32() {
    let mut writer = WriteBuffer::new();
    
    // Allocate space for pointer
    writer.allocate(4);
    
    // Write vector
    let values = vec![10, 20, 30, 40, 50];
    writer.write_vector_i32(0, &values);
    
    println!("Buffer size: {}", writer.size());
    println!("Binary: {:02x?}", writer.data());
    
    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_vector_i32(0);
    
    assert_eq!(values, read_values);
    println!("✓ Vector test passed: {:?}", read_values);
}

#[test]
fn test_vector_empty() {
    let mut writer = WriteBuffer::new();
    writer.allocate(4);
    
    let values: Vec<i32> = vec![];
    writer.write_vector_i32(0, &values);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_vector_i32(0);
    
    assert_eq!(values, read_values);
    assert!(read_values.is_empty());
    println!("✓ Empty vector test passed");
}

#[test]
fn test_vector_large() {
    let mut writer = WriteBuffer::new();
    writer.allocate(4);
    
    let values: Vec<i32> = (0..1000).collect();
    writer.write_vector_i32(0, &values);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_vector_i32(0);
    
    assert_eq!(values, read_values);
    println!("✓ Large vector test passed (1000 elements)");
}

