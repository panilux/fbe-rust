use fbe::buffer::{WriteBuffer, ReadBuffer};

#[test]
fn test_vector_f32() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(100);
    
    let values = vec![1.5_f32, 2.5_f32, 3.14159_f32];
    buffer.write_vector_f32(0, &values);
    
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let result = reader.read_vector_f32(0);
    
    assert_eq!(result.len(), 3);
    assert!((result[0] - 1.5).abs() < 0.001);
    assert!((result[2] - 3.14159).abs() < 0.001);
}

#[test]
fn test_array_f64() {
    let mut buffer = WriteBuffer::new();
    buffer.allocate(100);
    
    let values = vec![2.718281828_f64, 1.414213562_f64];
    buffer.write_array_f64(0, &values);
    
    let reader = ReadBuffer::from(buffer.data().to_vec());
    let result = reader.read_array_f64(0, 2);
    
    assert_eq!(result.len(), 2);
    assert!((result[0] - 2.718281828).abs() < 0.000001);
    assert!((result[1] - 1.414213562).abs() < 0.000001);
}

