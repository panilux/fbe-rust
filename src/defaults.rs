/*!
 * FBE Default Values Support for Rust
 */

use crate::buffer::{ReadBuffer, WriteBuffer};

// Numeric defaults
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub timeout: i32,
    pub retries: i32,
    pub threshold: f64,
    pub ratio: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: 30,     // Schema default
            retries: 3,      // Schema default
            threshold: 0.95, // Schema default
            ratio: 1.5,      // Schema default
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i32(offset, self.timeout);
        offset += 4;
        buffer.write_i32(offset, self.retries);
        offset += 4;
        buffer.write_f64(offset, self.threshold);
        offset += 8;
        buffer.write_f32(offset, self.ratio);
        offset += 4;
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let timeout = buffer.read_i32(offset);
        offset += 4;
        let retries = buffer.read_i32(offset);
        offset += 4;
        let threshold = buffer.read_f64(offset);
        offset += 8;
        let ratio = buffer.read_f32(offset);
        Self {
            timeout,
            retries,
            threshold,
            ratio,
        }
    }
}

// String and boolean defaults
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub enabled: bool,
    pub debug: bool,
    pub name: String,
    pub path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,                   // Schema default
            debug: false,                    // Schema default
            name: "DefaultName".to_string(), // Schema default
            path: "/var/log".to_string(),    // Schema default
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_bool(offset, self.enabled);
        offset += 1;
        buffer.write_bool(offset, self.debug);
        offset += 1;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_string(offset, &self.path);
        offset += 4 + self.path.len();
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let enabled = buffer.read_bool(offset);
        offset += 1;
        let debug = buffer.read_bool(offset);
        offset += 1;
        let name = buffer.read_string(offset);
        offset += 4 + name.len();
        let path = buffer.read_string(offset);
        Self {
            enabled,
            debug,
            name,
            path,
        }
    }
}

// Mixed defaults (some with, some without)
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: i32,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub tp: f64,
    pub sl: f64,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            id: 0,                 // Type default (no schema default)
            symbol: String::new(), // Type default (no schema default)
            price: 0.0,            // Schema default
            volume: 0.0,           // Schema default
            tp: 10.0,              // Schema default
            sl: -10.0,             // Schema default (negative)
        }
    }
}

impl Order {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i32(offset, self.id);
        offset += 4;
        buffer.write_string(offset, &self.symbol);
        offset += 4 + self.symbol.len();
        buffer.write_f64(offset, self.price);
        offset += 8;
        buffer.write_f64(offset, self.volume);
        offset += 8;
        buffer.write_f64(offset, self.tp);
        offset += 8;
        buffer.write_f64(offset, self.sl);
        offset += 8;
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let id = buffer.read_i32(offset);
        offset += 4;
        let symbol = buffer.read_string(offset);
        offset += 4 + symbol.len();
        let price = buffer.read_f64(offset);
        offset += 8;
        let volume = buffer.read_f64(offset);
        offset += 8;
        let tp = buffer.read_f64(offset);
        offset += 8;
        let sl = buffer.read_f64(offset);
        Self {
            id,
            symbol,
            price,
            volume,
            tp,
            sl,
        }
    }
}

// Optional fields with defaults
#[derive(Debug, Clone, PartialEq)]
pub struct OptionalFields {
    pub count: Option<i32>,
    pub text: Option<String>,
    pub flag: Option<bool>,
    pub value: Option<f64>,
}

impl Default for OptionalFields {
    fn default() -> Self {
        Self {
            count: None,                       // Schema default (null)
            text: Some("Default".to_string()), // Schema default
            flag: Some(true),                  // Schema default
            value: None,                       // Type default (no schema default)
        }
    }
}

impl OptionalFields {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_numeric_defaults() {
        let config = Config::new();
        assert_eq!(config.timeout, 30);
        assert_eq!(config.retries, 3);
        assert!((config.threshold - 0.95).abs() < 0.001);
        assert!((config.ratio - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_settings_string_bool_defaults() {
        let settings = Settings::new();
        assert_eq!(settings.enabled, true);
        assert_eq!(settings.debug, false);
        assert_eq!(settings.name, "DefaultName");
        assert_eq!(settings.path, "/var/log");
    }

    #[test]
    fn test_order_mixed_defaults() {
        let order = Order::new();
        assert_eq!(order.id, 0);
        assert_eq!(order.symbol, "");
        assert_eq!(order.price, 0.0);
        assert_eq!(order.volume, 0.0);
        assert_eq!(order.tp, 10.0);
        assert_eq!(order.sl, -10.0);
    }

    #[test]
    fn test_optional_fields_defaults() {
        let opt = OptionalFields::new();
        assert_eq!(opt.count, None);
        assert_eq!(opt.text, Some("Default".to_string()));
        assert_eq!(opt.flag, Some(true));
        assert_eq!(opt.value, None);
    }

    #[test]
    fn test_serialization_with_defaults() {
        let config = Config::new();
        let mut buffer = WriteBuffer::new();
        buffer.reserve(100);
        let size = config.serialize(&mut buffer);
        assert_eq!(size, 20); // 4 + 4 + 8 + 4

        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let config2 = Config::deserialize(&read_buffer);

        assert_eq!(config, config2);
    }

    #[test]
    fn test_modify_defaults() {
        let mut order = Order::new();
        assert_eq!(order.tp, 10.0);
        assert_eq!(order.sl, -10.0);

        order.tp = 20.0;
        order.sl = -20.0;

        assert_eq!(order.tp, 20.0);
        assert_eq!(order.sl, -20.0);
    }
}
