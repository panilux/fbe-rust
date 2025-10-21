use fbe::buffer::{ReadBuffer, WriteBuffer};
use std::fs;

#[test]
fn test_cross_platform_collections_v2() {
    let mut writer = WriteBuffer::new();
    writer.reserve(1024);

    let vector_values = vec![10, 20, 30];
    let map_entries = vec![(1, 100), (2, 200)];
    let set_values = vec![70, 80, 90];
    let array_values = [40, 50, 60];

    // Reserve space for pointers and inline array (match PHP layout)
    let vector_offset = 0;
    let map_offset = 4;
    let set_offset = 8;
    let array_offset = 16; // PHP uses offset 16
    writer.allocate(28); // 16 bytes (4 pointers) + 12 bytes (array)

    // Write array inline first
    writer.write_array_i32(array_offset, &array_values);

    // Write pointer-based collections
    writer.write_vector_i32(vector_offset, &vector_values);
    writer.write_map_i32(map_offset, &map_entries);
    writer.write_set_i32(set_offset, &set_values);

    fs::write("/tmp/rust_collections_v2.bin", writer.data()).unwrap();
    println!("Rust wrote {} bytes", writer.size());
    println!("Binary: {}", hex::encode(writer.data()));

    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());

    let read_vector = reader.read_vector_i32(vector_offset);
    let read_map = reader.read_map_i32(map_offset);
    let read_set = reader.read_set_i32(set_offset);
    let read_array = reader.read_array_i32(array_offset, 3);

    assert_eq!(vector_values, read_vector);
    assert_eq!(array_values.to_vec(), read_array);
    assert_eq!(map_entries, read_map);
    assert_eq!(set_values, read_set);

    println!("✓ All collections verified!");
}

