//! Fast Binary Encoding receiver

use crate::buffer::ReadBuffer;

/// Fast Binary Encoding base receiver
pub trait Receiver {
    /// Get logging flag
    fn is_logging(&self) -> bool;
    
    /// Enable/Disable logging
    fn set_logging(&mut self, enable: bool);
    
    /// Receive message handler (must be implemented)
    fn on_receive(&mut self, data: &[u8]) -> bool;
    
    /// Receive log message handler (can be overridden)
    fn on_receive_log(&self, message: &str) {
        // Default: do nothing
        let _ = message;
    }
    
    /// Receive and process data
    fn receive(&mut self, data: &[u8]) -> bool {
        let size = data.len();
        
        if self.is_logging() {
            self.on_receive_log(&format!("Received {} bytes", size));
        }
        
        self.on_receive(data)
    }
}

/// Trait for types that can be deserialized
pub trait Deserialize: Sized {
    fn deserialize(buffer: &ReadBuffer) -> Self;
}

