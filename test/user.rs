//! User struct

use crate::buffer::{{WriteBuffer, ReadBuffer}};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub side: Side,
}

impl User {
    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_i32(offset, self.id);
        offset += 4;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_i32(offset, self.side);
        offset += 4;
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        Self {
            id: { let val = buffer.read_i32(offset); offset += 4; val },
            name: { let val = buffer.read_string(offset); offset += 4 + val.len(); val },
            side: { let val = buffer.read_i32(offset); offset += 4; val },
        }
    }
}
