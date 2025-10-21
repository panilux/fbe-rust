use fbe::buffer::{WriteBuffer, ReadBuffer};
use std::fs;

#[test]
fn test_cross_platform_vector() {
    // Write Rust binary
    let mut writer = WriteBuffer::new();
    writer.allocate(4);
    
    let values = vec![100, 200, 300, 400, 500];
    writer.write_vector_i32(0, &values);
    
    fs::write("/tmp/rust_vector.bin", writer.data()).unwrap();
    println!("Rust wrote {} bytes", writer.size());
    println!("Rust binary: {:02x?}", writer.data());
    
    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_values = reader.read_vector_i32(0);
    assert_eq!(values, read_values);
    println!("✓ Rust round-trip: {:?}", read_values);
    
    // Try reading PHP binary if exists
    if std::path::Path::new("/tmp/php_vector.bin").exists() {
        println!("\nReading PHP binary...");
        let php_binary = fs::read("/tmp/php_vector.bin").unwrap();
        println!("PHP binary: {:02x?}", &php_binary);
        
        let mut php_reader = ReadBuffer::new();
        php_reader.attach_buffer(&php_binary, 0, php_binary.len());
        
        let php_values = php_reader.read_vector_i32(0);
        
        println!("PHP→Rust vector: {:?}", php_values);
        assert_eq!(values, php_values);
        
        println!("✓ Cross-platform vector test passed!");
    }
}

