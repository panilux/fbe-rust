use fbe::buffer::{ReadBuffer, WriteBuffer};
use std::fs;

#[test]
fn test_rust_write_read() {
    println!("============================================================");
    println!("Binary Compatibility Test: Rust");
    println!("============================================================\n");

    // Test 1: Rust Write
    println!("TEST 1: Rust Write");
    println!("------------------------------------------------------------");

    let mut buffer = WriteBuffer::new();
    buffer.allocate(100);

    let mut offset = 0;
    buffer.write_i32(offset, 42); // id
    offset += 4;

    buffer.write_string(offset, "EURUSD"); // name
    offset += 4 + 6;

    buffer.write_f64(offset, 1.23456); // value
    offset += 8;

    println!("Total written: {} bytes", offset);
    let binary = &buffer.data()[..offset];
    println!("Binary (hex): {}", hex::encode(binary));

    // Save
    fs::write("/tmp/simple_rust.bin", binary).unwrap();
    println!("✅ Saved to /tmp/simple_rust.bin\n");

    // Test 2: Rust Read
    println!("============================================================");
    println!("TEST 2: Rust Read");
    println!("============================================================");

    let reader = ReadBuffer::from(buffer.data().to_vec());
    let mut offset = 0;

    let id = reader.read_i32(offset);
    offset += 4;

    let name = reader.read_string(offset);
    offset += 4 + name.len();

    let value = reader.read_f64(offset);

    println!("ID: {}", id);
    println!("Name: {}", name);
    println!("Value: {}", value);

    assert_eq!(id, 42);
    assert_eq!(name, "EURUSD");
    assert!((value - 1.23456).abs() < 0.00001);

    println!("\n✅ Rust binary format working!");
}

#[test]
fn test_rust_read_php() {
    println!("\n============================================================");
    println!("THE CHALLENGE: Rust reads PHP binary");
    println!("============================================================\n");

    // Read PHP binary
    let binary = fs::read("/tmp/simple_php.bin").expect("Run PHP test first!");
    println!("PHP binary length: {} bytes", binary.len());
    println!("PHP binary (hex): {}", hex::encode(&binary));

    let reader = ReadBuffer::from(binary);
    let mut offset = 0;

    let id = reader.read_i32(offset);
    offset += 4;

    let name = reader.read_string(offset);
    offset += 4 + name.len();

    let value = reader.read_f64(offset);

    println!("\nRust reading PHP binary:");
    println!("ID: {}", id);
    println!("Name: {}", name);
    println!("Value: {}", value);

    assert_eq!(id, 42);
    assert_eq!(name, "EURUSD");
    assert!((value - 1.23456).abs() < 0.00001);

    println!("\n🎉 SUCCESS: Rust ↔ PHP binary compatible!");
}
