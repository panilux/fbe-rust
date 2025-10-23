use fbe::buffer::{ReadBuffer, WriteBuffer};

// Test flags (bitfields)
mod permissions {
    pub const NONE: i32 = 0x00;
    pub const READ: i32 = 0x01;
    pub const WRITE: i32 = 0x02;
    pub const EXECUTE: i32 = 0x04;
    pub const DELETE: i32 = 0x08;
    pub const ALL: i32 = READ | WRITE | EXECUTE | DELETE;
}

mod file_flags {
    pub const NONE: i32 = 0x00;
    pub const HIDDEN: i32 = 0x01;
    pub const SYSTEM: i32 = 0x02;
    pub const ARCHIVE: i32 = 0x04;
    pub const READONLY: i32 = 0x08;
    pub const COMPRESSED: i32 = 0x10;
}

#[test]
fn test_flags_single_bit() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    writer.write_i32(0, permissions::READ);
    writer.write_i32(4, permissions::WRITE);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 8);
    
    assert_eq!(permissions::READ, reader.read_i32(0));
    assert_eq!(permissions::WRITE, reader.read_i32(4));
}

#[test]
fn test_flags_combined() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    let flags = permissions::READ | permissions::WRITE;
    writer.write_i32(0, flags);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    let read_flags = reader.read_i32(0);
    
    assert_eq!(0x03, read_flags);
    assert!(read_flags & permissions::READ != 0);
    assert!(read_flags & permissions::WRITE != 0);
    assert!(read_flags & permissions::EXECUTE == 0);
}

#[test]
fn test_flags_all() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    writer.write_i32(0, permissions::ALL);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    let flags = reader.read_i32(0);
    
    assert!(flags & permissions::READ != 0);
    assert!(flags & permissions::WRITE != 0);
    assert!(flags & permissions::EXECUTE != 0);
    assert!(flags & permissions::DELETE != 0);
}

#[test]
fn test_flags_none() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    writer.write_i32(0, permissions::NONE);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    let flags = reader.read_i32(0);
    
    assert_eq!(0, flags);
    assert!(flags & permissions::READ == 0);
    assert!(flags & permissions::WRITE == 0);
}

#[test]
fn test_flags_in_struct() {
    // Struct: { id: 42, permissions: READ|WRITE, name: "file" }
    let mut writer = WriteBuffer::new();
    writer.allocate(100);
    
    writer.write_i32(0, 42);
    writer.write_i32(4, permissions::READ | permissions::WRITE);
    writer.write_string(8, "file");
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let id = reader.read_i32(0);
    let perms = reader.read_i32(4);
    let name = reader.read_string(8);
    
    assert_eq!(42, id);
    assert!(perms & permissions::READ != 0);
    assert!(perms & permissions::WRITE != 0);
    assert_eq!("file", name);
}

#[test]
fn test_multiple_flags_types() {
    let mut writer = WriteBuffer::new();
    writer.allocate(20);
    
    writer.write_i32(0, permissions::READ | permissions::EXECUTE);
    writer.write_i32(4, file_flags::HIDDEN | file_flags::READONLY);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 8);
    
    let perms = reader.read_i32(0);
    let file_fl = reader.read_i32(4);
    
    assert!(perms & permissions::READ != 0);
    assert!(perms & permissions::EXECUTE != 0);
    assert!(perms & permissions::WRITE == 0);
    
    assert!(file_fl & file_flags::HIDDEN != 0);
    assert!(file_fl & file_flags::READONLY != 0);
    assert!(file_fl & file_flags::ARCHIVE == 0);
}

#[test]
fn test_flags_operations() {
    // Test bitwise operations
    let mut flags = permissions::NONE;
    
    // Add READ
    flags |= permissions::READ;
    assert!(flags & permissions::READ != 0);
    
    // Add WRITE
    flags |= permissions::WRITE;
    assert!(flags & permissions::WRITE != 0);
    
    // Remove READ
    flags &= !permissions::READ;
    assert!(flags & permissions::READ == 0);
    assert!(flags & permissions::WRITE != 0);
    
    // Write and read
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    writer.write_i32(0, flags);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    let read_flags = reader.read_i32(0);
    
    assert_eq!(flags, read_flags);
}

#[test]
fn test_flags_array() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);
    
    let flags_array = vec![
        permissions::READ,
        permissions::READ | permissions::WRITE,
        permissions::ALL,
        permissions::NONE,
    ];
    
    writer.write_i32(0, flags_array.len() as i32);
    for (i, &flags) in flags_array.iter().enumerate() {
        writer.write_i32(4 + i * 4, flags);
    }
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let length = reader.read_i32(0) as usize;
    assert_eq!(4, length);
    
    for i in 0..length {
        let flags = reader.read_i32(4 + i * 4);
        assert_eq!(flags_array[i], flags);
    }
}

#[test]
fn test_flags_toggle() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    let mut flags = permissions::READ | permissions::WRITE;
    
    // Toggle WRITE (should remove it)
    flags ^= permissions::WRITE;
    assert!(flags & permissions::READ != 0);
    assert!(flags & permissions::WRITE == 0);
    
    // Toggle WRITE again (should add it back)
    flags ^= permissions::WRITE;
    assert!(flags & permissions::WRITE != 0);
    
    writer.write_i32(0, flags);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    assert_eq!(flags, reader.read_i32(0));
}

