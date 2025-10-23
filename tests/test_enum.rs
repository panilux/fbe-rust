use fbe::buffer::{ReadBuffer, WriteBuffer};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Pending = 0,
    Active = 1,
    Completed = 2,
    Cancelled = 3,
}

#[test]
fn test_enum_serialize_i32() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    writer.write_i32(0, Color::Red as i32);
    writer.write_i32(4, Color::Green as i32);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 8);
    
    let red = reader.read_i32(0);
    let green = reader.read_i32(4);
    
    assert_eq!(Color::Red as i32, red);
    assert_eq!(Color::Green as i32, green);
}

#[test]
fn test_enum_all_values() {
    let mut writer = WriteBuffer::new();
    writer.allocate(20);
    
    writer.write_i32(0, Color::Red as i32);
    writer.write_i32(4, Color::Green as i32);
    writer.write_i32(8, Color::Blue as i32);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 12);
    
    assert_eq!(0, reader.read_i32(0));
    assert_eq!(1, reader.read_i32(4));
    assert_eq!(2, reader.read_i32(8));
}

#[test]
fn test_enum_in_struct() {
    // Struct: { id: 42, status: Active, name: "test" }
    let mut writer = WriteBuffer::new();
    writer.allocate(100);
    
    writer.write_i32(0, 42); // id
    writer.write_i32(4, Status::Active as i32); // status enum
    writer.write_string(8, "test"); // name
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let id = reader.read_i32(0);
    let status = reader.read_i32(4);
    let name = reader.read_string(8);
    
    assert_eq!(42, id);
    assert_eq!(Status::Active as i32, status);
    assert_eq!("test", name);
}

#[test]
fn test_enum_array() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);
    
    let enums = vec![Status::Pending, Status::Active, Status::Completed];
    
    // Write array length
    writer.write_i32(0, enums.len() as i32);
    
    // Write enum values
    for (i, e) in enums.iter().enumerate() {
        writer.write_i32(4 + (i * 4), *e as i32);
    }
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, writer.size());
    
    let length = reader.read_i32(0) as usize;
    assert_eq!(3, length);
    
    let mut read_enums = Vec::new();
    for i in 0..length {
        read_enums.push(reader.read_i32(4 + i * 4));
    }
    
    assert_eq!(Status::Pending as i32, read_enums[0]);
    assert_eq!(Status::Active as i32, read_enums[1]);
    assert_eq!(Status::Completed as i32, read_enums[2]);
}

#[test]
fn test_enum_default_value() {
    let mut writer = WriteBuffer::new();
    writer.allocate(10);
    
    writer.write_i32(0, 0); // Default (Red)
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 4);
    
    let color = reader.read_i32(0);
    
    assert_eq!(Color::Red as i32, color);
}

#[test]
fn test_multiple_enum_types() {
    let mut writer = WriteBuffer::new();
    writer.allocate(20);
    
    writer.write_i32(0, Color::Blue as i32);
    writer.write_i32(4, Status::Completed as i32);
    writer.write_i32(8, Color::Green as i32);
    
    let mut reader = ReadBuffer::new();
    reader.attach_buffer(writer.data(), 0, 12);
    
    let color1 = reader.read_i32(0);
    let status = reader.read_i32(4);
    let color2 = reader.read_i32(8);
    
    assert_eq!(Color::Blue as i32, color1);
    assert_eq!(Status::Completed as i32, status);
    assert_eq!(Color::Green as i32, color2);
}

#[test]
fn test_enum_conversion() {
    // Test enum to/from i32 conversion
    let colors = vec![Color::Red, Color::Green, Color::Blue];
    
    for color in colors {
        let value = color as i32;
        let mut writer = WriteBuffer::new();
        writer.allocate(4);
        writer.write_i32(0, value);
        
        let mut reader = ReadBuffer::new();
        reader.attach_buffer(writer.data(), 0, 4);
        
        let read_value = reader.read_i32(0);
        
        assert_eq!(value, read_value);
    }
}

