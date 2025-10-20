//! FBE model trait for struct serialization

use crate::buffer::{ReadBuffer, WriteBuffer};

/// Base trait for FBE struct models
pub trait Model {
    /// The value type this model serializes
    type ValueType;
    
    /// Get FBE type ID
    fn fbe_type(&self) -> usize;
    
    /// Get FBE offset
    fn fbe_offset(&self) -> usize;
    
    /// Verify model
    fn verify(&self) -> bool;
    
    /// Get model size
    fn fbe_size(&self) -> usize;
    
    /// Serialize value and return serialized size
    fn serialize(&mut self, value: &Self::ValueType) -> usize;
    
    /// Deserialize value and return (value, deserialized size)
    fn deserialize(&self) -> (Self::ValueType, usize);
    
    /// Move to next position (for streaming)
    fn next(&mut self, size: usize);
    
    /// Get write buffer reference
    fn buffer_mut(&mut self) -> &mut WriteBuffer;
    
    /// Get read buffer reference
    fn buffer(&self) -> &ReadBuffer;
}

