//! Fast Binary Encoding sender

use crate::buffer::WriteBuffer;

/// Fast Binary Encoding base sender
pub trait Sender {
    /// Get logging flag
    fn is_logging(&self) -> bool;
    
    /// Enable/Disable logging
    fn set_logging(&mut self, enable: bool);
    
    /// Send message handler (must be implemented)
    fn on_send(&mut self, data: &[u8]) -> usize;
    
    /// Send log message handler (can be overridden)
    fn on_send_log(&self, message: &str) {
        // Default: do nothing
        let _ = message;
    }
    
    /// Send serialized data
    fn send_serialized(&mut self, data: &[u8]) -> usize {
        let size = data.len();
        
        if self.is_logging() {
            self.on_send_log(&format!("Sending {} bytes", size));
        }
        
        self.on_send(data)
    }
    
    /// Send a struct
    fn send<T>(&mut self, value: &T) -> usize 
    where
        T: Serialize + std::fmt::Display,
    {
        let mut buffer = WriteBuffer::new();
        value.serialize(&mut buffer);
        
        if self.is_logging() {
            self.on_send_log(&format!("Sending struct: {}", std::any::type_name::<T>()));
            self.on_send_log(&format!("{}", value));
        }
        
        self.send_serialized(buffer.data())
    }
}

/// Trait for types that can be serialized
pub trait Serialize {
    fn serialize(&self, buffer: &mut WriteBuffer) -> usize;
}

