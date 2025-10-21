use crate::buffer::{WriteBuffer, ReadBuffer};

#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub city: String,
    pub country: String,
}

impl Address {
    pub fn new(city: String, country: String) -> Self {
        Self { city, country }
    }
    
    /// Serialize Address to buffer at offset
    pub fn serialize(&self, buffer: &mut WriteBuffer, offset: usize) -> usize {
        let base_offset = offset + 8; // FBE struct header
        
        // Write struct header
        buffer.write_i32(offset, 0); // struct offset
        buffer.write_i32(offset + 4, 8); // struct size (2 string pointers)
        
        // Write city (pointer at base_offset)
        let city_ptr = buffer.size();
        buffer.write_u32(base_offset, city_ptr as u32);
        buffer.allocate(4 + self.city.len());
        buffer.write_u32(city_ptr, self.city.len() as u32);
        for (i, &b) in self.city.as_bytes().iter().enumerate() {
            buffer.write_i8(city_ptr + 4 + i, b as i8);
        }
        
        // Write country (pointer at base_offset + 4)
        let country_ptr = buffer.size();
        buffer.write_u32(base_offset + 4, country_ptr as u32);
        buffer.allocate(4 + self.country.len());
        buffer.write_u32(country_ptr, self.country.len() as u32);
        for (i, &b) in self.country.as_bytes().iter().enumerate() {
            buffer.write_i8(country_ptr + 4 + i, b as i8);
        }
        
        buffer.size()
    }
    
    /// Deserialize Address from buffer at offset
    pub fn deserialize(buffer: &ReadBuffer, offset: usize) -> Self {
        let base_offset = offset + 8;
        
        // Read city
        let city_ptr = buffer.read_u32(base_offset) as usize;
        let city_len = buffer.read_u32(city_ptr) as usize;
        let city_bytes = &buffer.data()[city_ptr + 4..city_ptr + 4 + city_len];
        let city = String::from_utf8_lossy(city_bytes).to_string();
        
        // Read country
        let country_ptr = buffer.read_u32(base_offset + 4) as usize;
        let country_len = buffer.read_u32(country_ptr) as usize;
        let country_bytes = &buffer.data()[country_ptr + 4..country_ptr + 4 + country_len];
        let country = String::from_utf8_lossy(country_bytes).to_string();
        
        Address { city, country }
    }
}

