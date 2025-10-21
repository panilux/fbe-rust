use fbe::buffer::{WriteBuffer, ReadBuffer};
use std::fs;

#[test]
fn test_cross_platform_collections_v2() {
    let mut writer = WriteBuffer::new();
    
    // Write all collections sequentially
    let vector_values = vec![10, 20, 30];
    let map_entries = vec![(1, 100), (2, 200)];
    let set_values = vec![70, 80, 90];
    let array_values = [40, 50, 60];
    
    // Track offsets
    let mut offset = 0;
    
    // 1. Vector
    writer.allocate(4);
    let vector_offset = offset;
    writer.write_vector_i32(vector_offset, &vector_values);
    offset += 4;
    
    // 2. Map
    writer.allocate(4);
    let map_offset = offset;
    writer.write_map_i32(map_offset, &map_entries);
    offset += 4;
    
    // 3. Set
    writer.allocate(4);
    let set_offset = offset;
    writer.write_set_i32(set_offset, &set_values);
    offset += 4;
    
    // 4. Array (inline)
    writer.allocate(12);
    let array_offset = offset;
    writer.write_array_i32(array_offset, &array_values);
    
    fs::write("/tmp/rust_collections_v2.bin", writer.data()).unwrap();
    println!("Rust wrote {} bytes", writer.size());
    println!("Offsets: vector={}, map={}, set={}, array={}", 
             vector_offset, map_offset, set_offset, array_offset);
    
    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let read_vector = reader.read_vector_i32(vector_offset);
    let read_map = reader.read_map_i32(map_offset);
    let read_set = reader.read_set_i32(set_offset);
    let read_array = reader.read_array_i32(array_offset, 3);
    
    assert_eq!(vector_values, read_vector);
    assert_eq!(map_entries, read_map);
    assert_eq!(set_values, read_set);
    assert_eq!(array_values.to_vec(), read_array);
    
    println!("✓ Rust round-trip passed");
    println!("  Vector: {:?}", read_vector);
    println!("  Map: {:?}", read_map);
    println!("  Set: {:?}", read_set);
    println!("  Array: {:?}", read_array);
    
    // Try reading PHP binary if exists
    if std::path::Path::new("/tmp/php_collections_v2.bin").exists() {
        println!("\nReading PHP binary...");
        let php_binary = fs::read("/tmp/php_collections_v2.bin").unwrap();
        
        let mut php_reader = ReadBuffer::new();
        php_reader.attach_buffer(&php_binary, 0, php_binary.len());
        
        let php_vector = php_reader.read_vector_i32(vector_offset);
        let php_map = php_reader.read_map_i32(map_offset);
        let php_set = php_reader.read_set_i32(set_offset);
        let php_array = php_reader.read_array_i32(array_offset, 3);
        
        println!("PHP→Rust Vector: {:?}", php_vector);
        println!("PHP→Rust Map: {:?}", php_map);
        println!("PHP→Rust Set: {:?}", php_set);
        println!("PHP→Rust Array: {:?}", php_array);
        
        assert_eq!(vector_values, php_vector);
        assert_eq!(map_entries, php_map);
        assert_eq!(set_values, php_set);
        assert_eq!(array_values.to_vec(), php_array);
        
        println!("✓ Cross-platform collections test passed!");
    }
}

