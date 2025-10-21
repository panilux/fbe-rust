/*!
 * FBE Struct Keys Support for Rust
 */

use crate::buffer::{ReadBuffer, WriteBuffer};
use std::hash::{Hash, Hasher};

// Single key field: Order
#[derive(Debug, Clone)]
pub struct Order {
    pub id: i32, // [key]
    pub symbol: String,
    pub price: f64,
}

impl Order {
    pub fn new(id: i32, symbol: String, price: f64) -> Self {
        Self { id, symbol, price }
    }

    pub fn key(&self) -> i32 {
        self.id
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i32(offset, self.id);
        offset += 4;
        buffer.write_string(offset, &self.symbol);
        offset += 4 + self.symbol.len();
        buffer.write_f64(offset, self.price);
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
        Self { id, symbol, price }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Order {}

impl Hash for Order {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// String key field: Balance
#[derive(Debug, Clone)]
pub struct Balance {
    pub currency: String, // [key]
    pub amount: f64,
}

impl Balance {
    pub fn new(currency: String, amount: f64) -> Self {
        Self { currency, amount }
    }

    pub fn key(&self) -> &str {
        &self.currency
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_string(offset, &self.currency);
        offset += 4 + self.currency.len();
        buffer.write_f64(offset, self.amount);
        offset += 8;
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let currency = buffer.read_string(offset);
        offset += 4 + currency.len();
        let amount = buffer.read_f64(offset);
        Self { currency, amount }
    }
}

impl PartialEq for Balance {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency
    }
}

impl Eq for Balance {}

impl Hash for Balance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
    }
}

// Composite key: UserSession
#[derive(Debug, Clone)]
pub struct UserSession {
    pub user_id: i32,       // [key]
    pub session_id: String, // [key]
    pub timestamp: i64,
    pub ip_address: String,
}

impl UserSession {
    pub fn new(user_id: i32, session_id: String, timestamp: i64, ip_address: String) -> Self {
        Self {
            user_id,
            session_id,
            timestamp,
            ip_address,
        }
    }

    pub fn key(&self) -> (i32, &str) {
        (self.user_id, &self.session_id)
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i32(offset, self.user_id);
        offset += 4;
        buffer.write_string(offset, &self.session_id);
        offset += 4 + self.session_id.len();
        buffer.write_i64(offset, self.timestamp);
        offset += 8;
        buffer.write_string(offset, &self.ip_address);
        offset += 4 + self.ip_address.len();
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let user_id = buffer.read_i32(offset);
        offset += 4;
        let session_id = buffer.read_string(offset);
        offset += 4 + session_id.len();
        let timestamp = buffer.read_i64(offset);
        offset += 8;
        let ip_address = buffer.read_string(offset);
        Self {
            user_id,
            session_id,
            timestamp,
            ip_address,
        }
    }
}

impl PartialEq for UserSession {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id && self.session_id == other.session_id
    }
}

impl Eq for UserSession {}

impl Hash for UserSession {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
        self.session_id.hash(state);
    }
}

// No key fields: LogEntry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: i64,
    pub message: String,
    pub level: String,
}

impl LogEntry {
    pub fn new(timestamp: i64, message: String, level: String) -> Self {
        Self {
            timestamp,
            message,
            level,
        }
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i64(offset, self.timestamp);
        offset += 8;
        buffer.write_string(offset, &self.message);
        offset += 4 + self.message.len();
        buffer.write_string(offset, &self.level);
        offset += 4 + self.level.len();
        buffer.set_size(offset);
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let timestamp = buffer.read_i64(offset);
        offset += 8;
        let message = buffer.read_string(offset);
        offset += 4 + message.len();
        let level = buffer.read_string(offset);
        Self {
            timestamp,
            message,
            level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_order_single_key() {
        let order1 = Order::new(123, "AAPL".to_string(), 150.50);
        let order2 = Order::new(123, "GOOGL".to_string(), 200.00);
        let order3 = Order::new(456, "AAPL".to_string(), 150.50);

        // Same key = equal
        assert_eq!(order1, order2);
        assert_ne!(order1, order3);

        // Key method
        assert_eq!(order1.key(), 123);
        assert_eq!(order3.key(), 456);
    }

    #[test]
    fn test_balance_string_key() {
        let balance1 = Balance::new("USD".to_string(), 1000.00);
        let balance2 = Balance::new("USD".to_string(), 2000.00);
        let balance3 = Balance::new("EUR".to_string(), 1000.00);

        assert_eq!(balance1, balance2);
        assert_ne!(balance1, balance3);

        assert_eq!(balance1.key(), "USD");
        assert_eq!(balance3.key(), "EUR");
    }

    #[test]
    fn test_user_session_composite_key() {
        let session1 = UserSession::new(
            100,
            "abc123".to_string(),
            1234567890,
            "192.168.1.1".to_string(),
        );
        let session2 = UserSession::new(
            100,
            "abc123".to_string(),
            9876543210,
            "10.0.0.1".to_string(),
        );
        let session3 = UserSession::new(
            100,
            "xyz789".to_string(),
            1234567890,
            "192.168.1.1".to_string(),
        );
        let session4 = UserSession::new(
            200,
            "abc123".to_string(),
            1234567890,
            "192.168.1.1".to_string(),
        );

        assert_eq!(session1, session2); // Same userId + sessionId
        assert_ne!(session1, session3); // Different sessionId
        assert_ne!(session1, session4); // Different userId

        assert_eq!(session1.key(), (100, "abc123"));
    }

    #[test]
    fn test_log_entry_no_keys() {
        let log1 = LogEntry::new(1234567890, "Test message".to_string(), "INFO".to_string());
        let log2 = LogEntry::new(1234567890, "Test message".to_string(), "INFO".to_string());
        let log3 = LogEntry::new(9876543210, "Other message".to_string(), "ERROR".to_string());

        // Default equality (all fields)
        assert_eq!(log1, log2);
        assert_ne!(log1, log3);
    }

    #[test]
    fn test_hash_map_usage() {
        let mut order_map: HashMap<i32, Order> = HashMap::new();

        let o1 = Order::new(1, "AAPL".to_string(), 100.00);
        let o2 = Order::new(2, "GOOGL".to_string(), 200.00);

        order_map.insert(o1.key(), o1.clone());
        order_map.insert(o2.key(), o2.clone());

        // Lookup by key
        assert!(order_map.contains_key(&1));
        assert_eq!(order_map.get(&1).unwrap().symbol, "AAPL");
    }

    #[test]
    fn test_hash_set_usage() {
        use std::collections::HashSet;

        let mut balance_set: HashSet<Balance> = HashSet::new();

        let b1 = Balance::new("USD".to_string(), 1000.00);
        let b2 = Balance::new("USD".to_string(), 2000.00); // Same key, different amount
        let b3 = Balance::new("EUR".to_string(), 1000.00);

        balance_set.insert(b1.clone());
        balance_set.insert(b2.clone()); // Should replace b1 (same key)
        balance_set.insert(b3.clone());

        // Only 2 items (USD and EUR)
        assert_eq!(balance_set.len(), 2);
        assert!(balance_set.contains(&b1)); // Contains USD
        assert!(balance_set.contains(&b3)); // Contains EUR
    }
}
