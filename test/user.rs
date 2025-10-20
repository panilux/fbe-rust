//! User struct

use fbe::buffer::{WriteBuffer, ReadBuffer};
use super::side::Side;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub side: Side,
}

impl User {
    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        // Calculate total size: id(4) + string_len(4) + string_data + side(1)
        let total_size = 4 + 4 + self.name.len() + 1;
        buffer.allocate(total_size);
        
        let mut offset = 0;
        buffer.write_i32(offset, self.id);
        offset += 4;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_i8(offset, self.side as i8);
        offset += 1;
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        Self {
            id: { let val = buffer.read_i32(offset); offset += 4; val },
            name: { let val = buffer.read_string(offset); offset += 4 + val.len(); val },
            side: unsafe { std::mem::transmute(buffer.read_i8(offset)) },
        }
    }
}
