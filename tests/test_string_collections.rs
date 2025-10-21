use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::field_model::FieldModel;
use fbe::field_model_collections::{
    FieldModelArrayString, FieldModelArrayStringMut, FieldModelVectorString,
    FieldModelVectorStringMut,
};

#[test]
fn test_field_model_vector_string() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4 + 100); // pointer + estimated data

    let values = vec![
        "Hello".to_string(),
        "Panilux".to_string(),
        "FBE".to_string(),
    ];
    let mut field = FieldModelVectorStringMut::new(&mut buffer, 0);
    field.set(&values);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelVectorString::new(reader.data(), 0);
    let result = field2.get();

    assert_eq!(result, values);
    assert_eq!(field2.size(), 4); // Pointer only
    assert!(field2.extra() > 0); // Dynamic data
}

#[test]
fn test_field_model_array_string() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(100);

    let values = vec!["Rust".to_string(), "PHP".to_string()];
    let mut field = FieldModelArrayStringMut::new(&mut buffer, 0, 2);
    field.set(&values);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelArrayString::new(reader.data(), 0, 2);
    let result = field2.get();

    assert_eq!(result, values);
    assert!(field2.size() > 0); // Variable size
}
