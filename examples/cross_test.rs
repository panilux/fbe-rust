use fbe::buffer::{WriteBuffer, ReadBuffer};

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

    println!("Original: id={}, name={}, side={:?}", user.id, user.name, user.side);

    // Serialize
    let mut buffer = WriteBuffer::new();
    let size = user.serialize(&mut buffer);
    
    println!("Serialized {} bytes", size);
    println!("Binary: {:?}", &buffer.buffer()[..size]);

    // Write to file for PHP to read
    std::fs::write("/tmp/rust_to_php.bin", &buffer.buffer()[..size]).unwrap();
    println!("\n✓ Wrote binary to /tmp/rust_to_php.bin");

    // Deserialize
    let read_buffer = ReadBuffer::new(buffer.buffer().to_vec());
    let decoded = test::user::User::deserialize(&read_buffer);

    println!("\nDecoded: id={}, name={}, side={:?}", decoded.id, decoded.name, decoded.side);
    
    assert_eq!(user.id, decoded.id);
    assert_eq!(user.name, decoded.name);
    
    println!("\n✓ Rust round-trip test passed!");
}

