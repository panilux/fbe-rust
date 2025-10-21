/*!
 * Cross-platform struct keys test: Rust ↔ PHP
 * HERSEY DAHA IYI BIR PANILUX ICIN! 🚀
 */

use fbe::buffer::{WriteBuffer, ReadBuffer};
use fbe::keys::{Order, Balance, UserSession};
use std::fs;

#[test]
fn test_order_rust_write_php_read() {
    let order = Order::new(123, "AAPL".to_string(), 150.50);

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    let size = order.serialize(&mut buffer);

    fs::write("/tmp/rust_order.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Order: {} bytes", size);
    println!("  Key: {}", order.key());
}

#[test]
fn test_balance_rust_write_php_read() {
    let balance = Balance::new("USD".to_string(), 1000.00);

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    balance.serialize(&mut buffer);

    fs::write("/tmp/rust_balance.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Balance");
    println!("  Key: {}", balance.key());
}

#[test]
fn test_user_session_rust_write_php_read() {
    let session = UserSession::new(
        100,
        "abc123".to_string(),
        1234567890,
        "192.168.1.1".to_string()
    );

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    session.serialize(&mut buffer);

    fs::write("/tmp/rust_session.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote UserSession");
    println!("  Key: {:?}", session.key());
}

#[test]
fn test_order_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_order.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let order = Order::deserialize(&buffer);

        assert_eq!(order.id, 123);
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.price, 150.50);
        assert_eq!(order.key(), 123);

        println!("✓ Rust read Order from PHP");
        println!("  Key: {}", order.key());
    }
}

#[test]
fn test_balance_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_balance.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let balance = Balance::deserialize(&buffer);

        assert_eq!(balance.currency, "USD");
        assert_eq!(balance.amount, 1000.00);
        assert_eq!(balance.key(), "USD");

        println!("✓ Rust read Balance from PHP");
        println!("  Key: {}", balance.key());
    }
}

#[test]
fn test_user_session_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_session.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let session = UserSession::deserialize(&buffer);

        assert_eq!(session.user_id, 100);
        assert_eq!(session.session_id, "abc123");
        assert_eq!(session.timestamp, 1234567890);
        assert_eq!(session.key(), (100, "abc123"));

        println!("✓ Rust read UserSession from PHP");
        println!("  Key: {:?}", session.key());
    }
}

