use fbe::buffer::{ReadBuffer, WriteBuffer};
use std::fs;

#[test]
fn test_rust_optional() {
    println!("============================================================");
    println!("Optional Types Test: Rust");
    println!("============================================================\n");

    // Test 1: Write optional values
    println!("TEST 1: Rust Write Optional Values");
    println!("------------------------------------------------------------");

    let mut buffer = WriteBuffer::new();
    buffer.allocate(100);

    let mut offset = 0;

    // Optional i32 with value
    buffer.write_optional_i32(offset, Some(42));
    let offset1 = offset;
    offset += 5;

    // Optional string with value
    buffer.write_optional_string(offset, Some("EURUSD"));
    let offset2 = offset;
    offset += 5;

    // Optional f64 NULL
    buffer.write_optional_f64(offset, None);
    let offset3 = offset;
    offset += 5;

    // Optional i32 NULL
    buffer.write_optional_i32(offset, None);

    println!("Total written: {} bytes", buffer.size());
    let binary = &buffer.data()[..buffer.size()];
    println!("Binary (hex): {}", hex::encode(binary));

    // Save for PHP
    fs::write("/tmp/optional_rust.bin", binary).unwrap();
    println!("✅ Saved to /tmp/optional_rust.bin\n");

    // Test 2: Read optional values
    println!("============================================================");
    println!("TEST 2: Rust Read Optional Values");
    println!("============================================================");

    let reader = ReadBuffer::from(buffer.data().to_vec());

    let val1 = reader.read_optional_i32(offset1);
    let val2 = reader.read_optional_string(offset2);
    let val3 = reader.read_optional_f64(offset3);
    let val4 = reader.read_optional_i32(offset3 + 5);

    println!("Optional i32: {:?}", val1);
    println!("Optional string: {:?}", val2);
    println!("Optional f64: {:?}", val3);
    println!("Optional i32 (null): {:?}", val4);

    assert_eq!(val1, Some(42));
    assert_eq!(val2, Some("EURUSD".to_string()));
    assert_eq!(val3, None);
    assert_eq!(val4, None);

    println!("\n✅ Rust optional types working!");
}

#[test]
fn test_rust_read_php_optional() {
    println!("\n============================================================");
    println!("Cross-Platform: Rust reads PHP optional");
    println!("============================================================\n");

    let binary = fs::read("/tmp/optional_php.bin").expect("Run PHP test first!");
    println!("PHP binary length: {} bytes", binary.len());

    let reader = ReadBuffer::from(binary);

    // PHP offsets based on actual binary structure
    println!("Debug: has_value at 0: {}", reader.has_value(0));
    println!("Debug: pointer at 1: {}", reader.read_u32(1));

    let val1 = reader.read_optional_i32(0); // offset 0
    let val2 = reader.read_optional_string(9); // offset 9 (0+5+4=9)
    let val3 = reader.read_optional_f64(14); // offset 14 (9+5=14, null)
    let val4 = reader.read_optional_i32(19); // offset 19 (14+5=19, null)

    println!("Rust reading PHP optional:");
    println!("Optional i32: {:?}", val1);
    println!("Optional string: {:?}", val2);
    println!("Optional f64: {:?}", val3);
    println!("Optional i32 (null): {:?}", val4);

    assert_eq!(val1, Some(42));
    assert_eq!(val2, Some("EURUSD".to_string()));
    assert_eq!(val3, None);
    assert_eq!(val4, None);

    println!("\n🎉 SUCCESS: Rust ↔ PHP optional types compatible!");
}
