use fbe::address::Address;
use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::user_with_address::UserWithAddress;

#[test]
fn test_nested_struct_write() {
    println!("============================================================");
    println!("Nested Struct Test: Rust");
    println!("============================================================\n");

    println!("TEST 1: Rust Write Nested Struct");
    println!("------------------------------------------------------------");

    let mut buffer = WriteBuffer::new();
    buffer.allocate(200);

    let address = Address::new("Istanbul".to_string(), "Turkey".to_string());
    let user = UserWithAddress::new(42, "Panilux".to_string(), address);

    user.serialize(&mut buffer, 0);

    println!("Total written: {} bytes", buffer.size());
    let binary = &buffer.data()[..buffer.size()];
    println!("Binary (hex): {}", hex::encode(binary));

    // Save for PHP
    std::fs::write("/tmp/nested_rust.bin", binary).unwrap();
    println!("✅ Saved to /tmp/nested_rust.bin\n");
}

#[test]
fn test_nested_struct_read() {
    println!("============================================================");
    println!("TEST 2: Rust Read Nested Struct");
    println!("============================================================");

    // Create test data
    let mut buffer = WriteBuffer::new();
    buffer.allocate(200);

    let address = Address::new("Istanbul".to_string(), "Turkey".to_string());
    let user = UserWithAddress::new(42, "Panilux".to_string(), address);
    user.serialize(&mut buffer, 0);

    // Read back
    let read_buffer = ReadBuffer::from(buffer.data()[..buffer.size()].to_vec());
    let read_user = UserWithAddress::deserialize(&read_buffer, 0);

    println!("User ID: {}", read_user.id);
    println!("User Name: {}", read_user.name);
    println!("Address City: {}", read_user.address.city);
    println!("Address Country: {}", read_user.address.country);

    assert_eq!(read_user.id, 42);
    assert_eq!(read_user.name, "Panilux");
    assert_eq!(read_user.address.city, "Istanbul");
    assert_eq!(read_user.address.country, "Turkey");

    println!("\n✅ Rust nested struct working!");
}

#[test]
fn test_cross_platform_nested() {
    println!("============================================================");
    println!("TEST 3: Cross-Platform Nested Struct (Rust → PHP)");
    println!("============================================================");

    // Read PHP binary
    let php_binary = std::fs::read("/tmp/nested_php.bin").expect("PHP binary not found");
    println!("PHP binary length: {} bytes", php_binary.len());

    let read_buffer = ReadBuffer::from(php_binary);
    let user = UserWithAddress::deserialize(&read_buffer, 0);

    println!("User ID: {}", user.id);
    println!("User Name: {}", user.name);
    println!("Address City: {}", user.address.city);
    println!("Address Country: {}", user.address.country);

    assert_eq!(user.id, 42);
    assert_eq!(user.name, "Panilux");
    assert_eq!(user.address.city, "Istanbul");
    assert_eq!(user.address.country, "Turkey");

    println!("\n✅ Cross-platform nested struct working!");
}
