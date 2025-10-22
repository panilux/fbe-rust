use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::field_model::{FieldModelI32, FieldModelI32Mut, FieldModelOptional, FieldModelOptionalMut};
use fbe::final_model::{FinalModel, FinalModelI32, FinalModelI32Mut, FinalModelOptional, FinalModelOptionalMut};

#[test]
fn test_field_model_optional_i32_with_value() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);

    // Create optional i32 field model
    let optional_model: FieldModelOptionalMut<i32, _> = FieldModelOptionalMut::new(&mut writer, 0, |buf, off| FieldModelI32Mut::new(buf, off));
    
    // Set value
    optional_model.set_some(42, |model, value| model.set(value));

    // Read back
    let data = writer.data();
    let optional_model_read: FieldModelOptional<i32, _> = FieldModelOptional::new(&data, 0, |buf, off| FieldModelI32::new(buf, off));

    assert!(optional_model_read.has_value());
    let value = optional_model_read.get(|model| model.get());
    assert_eq!(value, Some(42));
}

#[test]
fn test_field_model_optional_i32_null() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);

    // Create optional i32 field model
    let mut optional_model: FieldModelOptionalMut<i32, _> = FieldModelOptionalMut::new(&mut writer, 0, |buf, off| FieldModelI32Mut::new(buf, off));
    
    // Set null
    optional_model.set_none();

    // Read back
    let data = writer.data();
    let optional_model_read: FieldModelOptional<i32, _> = FieldModelOptional::new(&data, 0, |buf, off| FieldModelI32::new(buf, off));

    assert!(!optional_model_read.has_value());
    let value = optional_model_read.get(|model| model.get());
    assert_eq!(value, None);
}

#[test]
fn test_final_model_optional_i32_with_value() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);

    // Create optional i32 final model
    let optional_model: FinalModelOptionalMut<i32, _> = FinalModelOptionalMut::new(&mut writer, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    
    // Set value
    optional_model.set_some(99, |model, value| model.set(value));

    // Read back
    let data = writer.data();
    let optional_model_read: FinalModelOptional<i32, _> = FinalModelOptional::new(&data, 0, |buf, off| FinalModelI32::new(buf, off));

    assert!(optional_model_read.has_value());
    let value = optional_model_read.get(|model| model.get());
    assert_eq!(value, Some(99));
}

#[test]
fn test_final_model_optional_i32_null() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);

    // Create optional i32 final model
    let mut optional_model: FinalModelOptionalMut<i32, _> = FinalModelOptionalMut::new(&mut writer, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    
    // Set null
    optional_model.set_none();

    // Read back
    let data = writer.data();
    let optional_model_read: FinalModelOptional<i32, _> = FinalModelOptional::new(&data, 0, |buf, off| FinalModelI32::new(buf, off));

    assert!(!optional_model_read.has_value());
    let value = optional_model_read.get(|model| model.get());
    assert_eq!(value, None);
}

#[test]
fn test_optional_size() {
    let mut writer = WriteBuffer::new();
    writer.allocate(100);

    // Test with value
    let optional_model: FinalModelOptionalMut<i32, _> = FinalModelOptionalMut::new(&mut writer, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    optional_model.set_some(42, |model, value| model.set(value));

    let data = writer.data();
    let optional_model_read: FinalModelOptional<i32, _> = FinalModelOptional::new(&data, 0, |buf, off| FinalModelI32::new(buf, off));
    
    // Size should be 1 (flag) + 4 (i32) = 5
    assert_eq!(optional_model_read.size(), 5);

    // Test with null
    let mut writer2 = WriteBuffer::new();
    writer2.allocate(100);
    let mut optional_model2: FinalModelOptionalMut<i32, _> = FinalModelOptionalMut::new(&mut writer2, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    optional_model2.set_none();

    let data2 = writer2.data();
    let optional_model_read2: FinalModelOptional<i32, _> = FinalModelOptional::new(&data2, 0, |buf, off| FinalModelI32::new(buf, off));
    
    // Size should be 1 (flag only)
    assert_eq!(optional_model_read2.size(), 1);
}


