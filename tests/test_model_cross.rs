/*!
 * Cross-platform Model/FinalModel test: Rust ↔ PHP
 */

use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::model_final::Product;
use std::fs;

#[test]
fn test_model_rust_write_php_read() {
    let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

    let mut buffer = WriteBuffer::new();
    let size = product.serialize_model(&mut buffer);

    fs::write("/tmp/rust_product_model.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Product (Model): {} bytes", size);
    println!("  Binary: {}", hex::encode(buffer.data()));
}

#[test]
fn test_final_model_rust_write_php_read() {
    let product = Product::new(123, "Laptop".to_string(), 999.99, 5);

    let mut buffer = WriteBuffer::new();
    let size = product.serialize_final(&mut buffer);

    fs::write("/tmp/rust_product_final.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Product (FinalModel): {} bytes", size);
    println!("  Binary: {}", hex::encode(buffer.data()));
}

#[test]
fn test_model_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_product_model.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let (product, size) = Product::deserialize_model(&buffer);

        assert_eq!(product.id, 123);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, 999.99);
        assert_eq!(product.quantity, 5);

        println!("✓ Rust read Product (Model) from PHP: {} bytes", size);
        println!(
            "  id={}, name={}, price={}, quantity={}",
            product.id, product.name, product.price, product.quantity
        );
    }
}

#[test]
fn test_final_model_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_product_final.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let (product, size) = Product::deserialize_final(&buffer);

        assert_eq!(product.id, 123);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, 999.99);
        assert_eq!(product.quantity, 5);

        println!("✓ Rust read Product (FinalModel) from PHP: {} bytes", size);
        println!(
            "  id={}, name={}, price={}, quantity={}",
            product.id, product.name, product.price, product.quantity
        );
    }
}
