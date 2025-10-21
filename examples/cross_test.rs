use fbe::buffer::{ReadBuffer, WriteBuffer};

mod test {
    include!("../test/mod.rs");
}

fn main() {
    println!("=== Rust FBE Cross-Platform Test ===\n");

    // Create test data
    let user = test::user::User {
        id: 42,
        name: "Panilux".to_string(),
        side: test::side::Side::Buy,
    };

    println!(
        "Original: id={}, name={}, side={:?}",
        user.id, user.name, user.side
    );

    // Serialize
    let mut buffer = WriteBuffer::new();
    let size = user.serialize(&mut buffer);

    println!("Serialized {} bytes", size);
    println!("Binary: {:?}", buffer.data());

    // Write to file for PHP to read
    std::fs::write("/tmp/rust_to_php.bin", buffer.data()).unwrap();
    println!("\n✓ Wrote binary to /tmp/rust_to_php.bin");

    // Deserialize
    let mut read_buffer = ReadBuffer::new();
    read_buffer.attach_buffer(buffer.data(), 0, buffer.size());
    let decoded = test::user::User::deserialize(&read_buffer);

    println!(
        "\nDecoded: id={}, name={}, side={:?}",
        decoded.id, decoded.name, decoded.side
    );

    assert_eq!(user.id, decoded.id);
    assert_eq!(user.name, decoded.name);

    println!("\n✓ Rust round-trip test passed!");

    // Try reading PHP binary if exists
    if std::path::Path::new("/tmp/php_to_rust.bin").exists() {
        println!("\n=== Reading PHP Binary ===");
        let php_binary = std::fs::read("/tmp/php_to_rust.bin").unwrap();
        let mut php_buffer = ReadBuffer::new();
        php_buffer.attach_buffer(&php_binary, 0, php_binary.len());
        let php_user = test::user::User::deserialize(&php_buffer);

        println!(
            "PHP→Rust: id={}, name={}, side={:?}",
            php_user.id, php_user.name, php_user.side
        );
        println!("✓ Successfully read PHP binary!");
    }
}
