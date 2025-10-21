use crate::buffer::{WriteBuffer, ReadBuffer};
use crate::address::Address;

#[derive(Debug, Clone, PartialEq)]
pub struct UserWithAddress {
    pub id: i32,
    pub name: String,
    pub address: Address,
}

impl UserWithAddress {
    pub fn new(id: i32, name: String, address: Address) -> Self {
        Self { id, name, address }
    }
    
    /// Serialize UserWithAddress to buffer
    pub fn serialize(&self, buffer: &mut WriteBuffer, offset: usize) -> usize {
        let base_offset = offset + 8; // FBE struct header
        
        // Write struct header
        buffer.write_i32(offset, 0);
        buffer.write_i32(offset + 4, 24); // struct size: 4 (id) + 4 (name ptr) + 16 (address)
        
        // Write id
        buffer.write_i32(base_offset, self.id);
        
        // Write name (pointer at base_offset + 4)
        let name_ptr = buffer.size();
        buffer.write_u32(base_offset + 4, name_ptr as u32);
        buffer.allocate(4 + self.name.len());
        buffer.write_u32(name_ptr, self.name.len() as u32);
        for (i, &b) in self.name.as_bytes().iter().enumerate() {
            buffer.write_i8(name_ptr + 4 + i, b as i8);
        }
        
        // Write address (nested struct at base_offset + 8)
        self.address.serialize(buffer, base_offset + 8);
        
        buffer.size()
    }
    
    /// Deserialize UserWithAddress from buffer
    pub fn deserialize(buffer: &ReadBuffer, offset: usize) -> Self {
        let base_offset = offset + 8;
        
        // Read id
        let id = buffer.read_i32(base_offset);
        
        // Read name
        let name_ptr = buffer.read_u32(base_offset + 4) as usize;
        let name_len = buffer.read_u32(name_ptr) as usize;
        let name_bytes = &buffer.data()[name_ptr + 4..name_ptr + 4 + name_len];
        let name = String::from_utf8_lossy(name_bytes).to_string();
        
        // Read address (nested struct)
        let address = Address::deserialize(buffer, base_offset + 8);
        
        UserWithAddress { id, name, address }
    }
}

