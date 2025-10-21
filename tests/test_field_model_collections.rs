use fbe::buffer::{ReadBuffer, WriteBuffer};
use fbe::field_model::FieldModel;
use fbe::field_model_collections::*;
use std::collections::{HashMap, HashSet};

#[test]
fn test_field_model_vector_i32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4 + 4 + 12); // pointer + size + 3 elements

    let mut field = FieldModelVectorI32Mut::new(&mut buffer, 0);
    field.set(&[10, 20, 30]);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelVectorI32::new(reader.data(), 0);
    let values = field2.get();

    assert_eq!(values, vec![10, 20, 30]);
    assert_eq!(field2.size(), 4); // Pointer only
    assert_eq!(field2.extra(), 16); // size + 3 elements
}

#[test]
fn test_field_model_array_i32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(12); // 3 × 4 bytes

    let mut field = FieldModelArrayI32Mut::new(&mut buffer, 0, 3);
    field.set(&[100, 200, 300]);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelArrayI32::new(reader.data(), 0, 3);
    let values = field2.get();

    assert_eq!(values, vec![100, 200, 300]);
    assert_eq!(field2.size(), 12); // 3 × 4
}

#[test]
fn test_field_model_map_i32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4 + 4 + 16); // pointer + size + 2 pairs

    let mut map = HashMap::new();
    map.insert(1, 100);
    map.insert(2, 200);

    let mut field = FieldModelMapI32Mut::new(&mut buffer, 0);
    field.set(&map);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelMapI32::new(reader.data(), 0);
    let result = field2.get();

    assert_eq!(result.len(), 2);
    assert_eq!(result.get(&1), Some(&100));
    assert_eq!(result.get(&2), Some(&200));
    assert_eq!(field2.size(), 4); // Pointer only
    assert_eq!(field2.extra(), 20); // size + 2 pairs
}

#[test]
fn test_field_model_set_i32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(4 + 4 + 12); // pointer + size + 3 elements

    let mut set = HashSet::new();
    set.insert(5);
    set.insert(10);
    set.insert(15);

    let mut field = FieldModelSetI32Mut::new(&mut buffer, 0);
    field.set(&set);

    // Read back
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let field2 = FieldModelSetI32::new(reader.data(), 0);
    let result = field2.get();

    assert_eq!(result.len(), 3);
    assert!(result.contains(&5));
    assert!(result.contains(&10));
    assert!(result.contains(&15));
    assert_eq!(field2.size(), 4); // Pointer only
    assert_eq!(field2.extra(), 16); // size + 3 elements
}
