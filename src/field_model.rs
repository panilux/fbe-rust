//! FBE field model traits and implementations

/// Base trait for FBE field models
pub trait FieldModel {
    /// Get FBE type ID
    fn fbe_type(&self) -> usize;
    
    /// Get FBE offset
    fn fbe_offset(&self) -> usize;
    
    /// Get FBE size (fixed size for primitives)
    fn fbe_size(&self) -> usize;
    
    /// Get FBE extra size (for variable-length types)
    fn fbe_extra(&self) -> usize {
        0
    }
    
    /// Verify field
    fn verify(&self) -> bool;
}

