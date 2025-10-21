use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::field_model::{
    FieldModel, FieldModelBool, FieldModelBoolMut, FieldModelI32, FieldModelI32Mut,
    FieldModelString, FieldModelStringMut,
};

#[test]
fn test_field_model_bool() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(1);

    let mut field = FieldModelBoolMut::new(&mut buffer, 0);
    field.set(true);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelBool::new(reader.data(), 0);
    assert_eq!(field2.get(), true);
    assert_eq!(field2.size(), 1);
}

#[test]
fn test_field_model_i32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4);

    let mut field = FieldModelI32Mut::new(&mut buffer, 0);
    field.set(42);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelI32::new(reader.data(), 0);
    assert_eq!(field2.get(), 42);
    assert_eq!(field2.size(), 4);
}

#[test]
fn test_field_model_string() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4 + 7); // size + "Panilux"

    let mut field = FieldModelStringMut::new(&mut buffer, 0);
    field.set("Panilux");

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelString::new(reader.data(), 0);
    assert_eq!(field2.get(), "Panilux");
    assert_eq!(field2.size(), 4); // Size prefix only
    assert_eq!(field2.extra(), 7); // String length
}

#[test]
fn test_field_model_offset() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(8);

    // Write two i32 values
    let mut field1 = FieldModelI32Mut::new(&mut buffer, 0);
    field1.set(100);

    let mut field2 = FieldModelI32Mut::new(&mut buffer, 4);
    field2.set(200);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let f1 = FieldModelI32::new(reader.data(), 0);
    let f2 = FieldModelI32::new(reader.data(), 4);

    assert_eq!(f1.get(), 100);
    assert_eq!(f2.get(), 200);
}
