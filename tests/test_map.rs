use fbe::buffer::{ReadBuffer, WriteBuffer};

#[test]
fn test_map_i32() {
    let mut writer = WriteBuffer::new();
    writer.allocate(4);

    // Write map
    let entries = vec![(1, 100), (2, 200), (3, 300)];
    writer.write_map_i32(0, &entries);

    println!("Buffer size: {}", writer.size());
    println!("Binary: {:02x?}", writer.data());

    // Read back
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());

    let read_entries = reader.read_map_i32(0);

    assert_eq!(entries, read_entries);
    println!("✓ Map test passed: {:?}", read_entries);
}

#[test]
fn test_map_empty() {
    let mut writer = WriteBuffer::new();
    writer.allocate(4);

    let entries: Vec<(i32, i32)> = vec![];
    writer.write_map_i32(0, &entries);

    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());

    let read_entries = reader.read_map_i32(0);

    assert_eq!(entries, read_entries);
    assert!(read_entries.is_empty());
    println!("✓ Empty map test passed");
}
