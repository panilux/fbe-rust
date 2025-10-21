/*!
 * Cross-platform default values test: Rust ↔ PHP
 */

use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::defaults::{Config, Order, Settings};
use std::fs;

#[test]
fn test_config_rust_write_php_read() {
    let config = Config::new(); // Uses defaults

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    config.serialize(&mut buffer);

    fs::write("/tmp/rust_config.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Config with defaults");
    println!(
        "  timeout={}, retries={}, threshold={}, ratio={}",
        config.timeout, config.retries, config.threshold, config.ratio
    );
}

#[test]
fn test_settings_rust_write_php_read() {
    let settings = Settings::new(); // Uses defaults

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    settings.serialize(&mut buffer);

    fs::write("/tmp/rust_settings.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Settings with defaults");
    println!(
        "  enabled={}, debug={}, name={}, path={}",
        settings.enabled, settings.debug, settings.name, settings.path
    );
}

#[test]
fn test_order_rust_write_php_read() {
    let order = Order::new(); // Uses defaults

    let mut buffer = WriteBuffer::new();
    buffer.reserve(100);
    order.serialize(&mut buffer);

    fs::write("/tmp/rust_order_defaults.bin", buffer.data()).expect("Failed to write");

    println!("✓ Rust wrote Order with defaults");
    println!("  tp={}, sl={}", order.tp, order.sl);
}

#[test]
fn test_config_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_config.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let config = Config::deserialize(&buffer);

        assert_eq!(config.timeout, 30);
        assert_eq!(config.retries, 3);
        assert!((config.threshold - 0.95).abs() < 0.001);

        println!("✓ Rust read Config from PHP");
        println!(
            "  timeout={}, retries={}, threshold={}",
            config.timeout, config.retries, config.threshold
        );
    }
}

#[test]
fn test_settings_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_settings.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let settings = Settings::deserialize(&buffer);

        assert_eq!(settings.enabled, true);
        assert_eq!(settings.debug, false);
        assert_eq!(settings.name, "DefaultName");
        assert_eq!(settings.path, "/var/log");

        println!("✓ Rust read Settings from PHP");
        println!("  name={}, path={}", settings.name, settings.path);
    }
}

#[test]
fn test_order_php_write_rust_read() {
    if let Ok(data) = fs::read("/tmp/php_order_defaults.bin") {
        let mut buffer = ReadBuffer::new();
        buffer.attach_buffer(&data, 0, data.len());
        let order = Order::deserialize(&buffer);

        assert_eq!(order.tp, 10.0);
        assert_eq!(order.sl, -10.0);

        println!("✓ Rust read Order from PHP");
        println!("  tp={}, sl={}", order.tp, order.sl);
    }
}
