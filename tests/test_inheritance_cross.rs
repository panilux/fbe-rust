/*!
 * Cross-platform inheritance test: Rust ↔ PHP
 */

use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::inheritance::{Employee, Manager, Person};
use std::fs;

#[test]
fn test_rust_write_php_read() {
    // Create Manager in Rust
    let manager = Manager::new(
        "Charlie".to_string(),
        40,
        "Panilux".to_string(),
        95000.75,
        10,
    );

    // Serialize to binary
    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    let size = manager.serialize(&mut buffer);

    // Save to file for PHP to read
    fs::write("/tmp/rust_manager.bin", buffer.data()).expect("Failed to write file");

    println!("✓ Rust wrote Manager: {} bytes", size);
    println!("  Binary: {}", hex::encode(buffer.data()));
}

#[test]
fn test_php_write_rust_read() {
    // Read binary written by PHP
    let data = fs::read("/tmp/php_manager.bin").expect("Failed to read PHP file");

    // Deserialize in Rust
    let mut buffer = ReadBuffer::new();
    buffer.attach_buffer(&data, 0, data.len());
    let manager = Manager::deserialize(&buffer);

    // Verify
    assert_eq!(manager.name, "Charlie");
    assert_eq!(manager.age, 40);
    assert_eq!(manager.company, "Panilux");
    assert_eq!(manager.salary, 95000.75);
    assert_eq!(manager.team_size, 10);

    println!("✓ Rust read Manager from PHP");
    println!(
        "  Name: {}, Age: {}, Company: {}, Salary: {}, Team: {}",
        manager.name, manager.age, manager.company, manager.salary, manager.team_size
    );
}

#[test]
fn test_employee_cross_platform() {
    // Rust → PHP
    let employee = Employee::new("Bob".to_string(), 35, "Panilux".to_string(), 75000.50);

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    employee.serialize(&mut buffer);
    fs::write("/tmp/rust_employee.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Employee");

    // PHP → Rust
    let data = fs::read("/tmp/php_employee.bin").expect("Failed to read PHP file");
    let mut read_buffer = ReadBuffer::new();
    read_buffer.attach_buffer(&data, 0, data.len());
    let employee2 = Employee::deserialize(&read_buffer);

    assert_eq!(employee2.name, "Bob");
    assert_eq!(employee2.age, 35);
    assert_eq!(employee2.company, "Panilux");
    assert_eq!(employee2.salary, 75000.50);

    println!("✓ Rust read Employee from PHP");
}

#[test]
fn test_person_cross_platform() {
    // Rust → PHP
    let person = Person::new("Alice".to_string(), 30);

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    person.serialize(&mut buffer);
    fs::write("/tmp/rust_person.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Person");

    // PHP → Rust
    let data = fs::read("/tmp/php_person.bin").expect("Failed to read PHP file");
    let mut read_buffer = ReadBuffer::new();
    read_buffer.attach_buffer(&data, 0, data.len());
    let person2 = Person::deserialize(&read_buffer);

    assert_eq!(person2.name, "Alice");
    assert_eq!(person2.age, 30);

    println!("✓ Rust read Person from PHP");
}
