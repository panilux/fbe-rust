use fbe::buffer::{WriteBuffer, ReadBuffer};
use fbe::field_model::{FieldModelI32, FieldModelI32Mut, FieldModelOptional, FieldModelOptionalMut};
use fbe::final_model::{FinalModelI32, FinalModelI32Mut, FinalModelOptional, FinalModelOptionalMut};
use std::fs;

#[test]
fn test_optional_cross_platform() {
    println!("\n=== Rust → PHP Optional Compatibility Test ===\n");

    // Test 1: FieldModel Optional with value
    println!("Test 1: FieldModel Optional<i32> with value (42)");
    let mut writer1 = WriteBuffer::new();
    writer1.allocate(100);
    let optional_model1: FieldModelOptionalMut<i32, _> = 
        FieldModelOptionalMut::new(&mut writer1, 0, |buf, off| FieldModelI32Mut::new(buf, off));
    optional_model1.set_some(42, |model, value| model.set(value));
    let data1 = writer1.data();
    println!("Rust serialized: {}", hex::encode(&data1));
    fs::write("/tmp/rust_optional_field_value.bin", &data1).unwrap();
    println!("Saved to: /tmp/rust_optional_field_value.bin\n");

    // Test 2: FieldModel Optional null
    println!("Test 2: FieldModel Optional<i32> null");
    let mut writer2 = WriteBuffer::new();
    writer2.allocate(100);
    let mut optional_model2: FieldModelOptionalMut<i32, _> = 
        FieldModelOptionalMut::new(&mut writer2, 0, |buf, off| FieldModelI32Mut::new(buf, off));
    optional_model2.set_none();
    let data2 = writer2.data();
    println!("Rust serialized: {}", hex::encode(&data2));
    fs::write("/tmp/rust_optional_field_null.bin", &data2).unwrap();
    println!("Saved to: /tmp/rust_optional_field_null.bin\n");

    // Test 3: FinalModel Optional with value
    println!("Test 3: FinalModel Optional<i32> with value (99)");
    let mut writer3 = WriteBuffer::new();
    writer3.allocate(100);
    let optional_model3: FinalModelOptionalMut<i32, _> = 
        FinalModelOptionalMut::new(&mut writer3, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    optional_model3.set_some(99, |model, value| model.set(value));
    let data3 = writer3.data();
    println!("Rust serialized: {}", hex::encode(&data3));
    fs::write("/tmp/rust_optional_final_value.bin", &data3).unwrap();
    println!("Saved to: /tmp/rust_optional_final_value.bin\n");

    // Test 4: FinalModel Optional null
    println!("Test 4: FinalModel Optional<i32> null");
    let mut writer4 = WriteBuffer::new();
    writer4.allocate(100);
    let mut optional_model4: FinalModelOptionalMut<i32, _> = 
        FinalModelOptionalMut::new(&mut writer4, 0, |buf, off| FinalModelI32Mut::new(buf, off));
    optional_model4.set_none();
    let data4 = writer4.data();
    println!("Rust serialized: {}", hex::encode(&data4));
    fs::write("/tmp/rust_optional_final_null.bin", &data4).unwrap();
    println!("Saved to: /tmp/rust_optional_final_null.bin\n");

    println!("=== PHP → Rust Optional Compatibility Test ===\n");

    // Read PHP-generated files
    let test_files = vec![
        ("/tmp/php_optional_field_value.bin", "FieldModel Optional<i32> with value", true, 42),
        ("/tmp/php_optional_field_null.bin", "FieldModel Optional<i32> null", false, 0),
        ("/tmp/php_optional_final_value.bin", "FinalModel Optional<i32> with value", true, 99),
        ("/tmp/php_optional_final_null.bin", "FinalModel Optional<i32> null", false, 0),
    ];

    for (file, description, should_have_value, expected_value) in test_files {
        if let Ok(data) = fs::read(file) {
            println!("Reading: {}", description);
            println!("PHP data: {}", hex::encode(&data));

            if file.contains("field_value") {
                let optional_model: FieldModelOptional<i32, _> = 
                    FieldModelOptional::new(&data, 0, |buf, off| FieldModelI32::new(buf, off));
                println!("Has value: {}", optional_model.has_value());
                if optional_model.has_value() {
                    let value = optional_model.get(|model| model.get()).unwrap();
                    println!("Value: {}", value);
                    assert_eq!(value, expected_value);
                }
                assert_eq!(optional_model.has_value(), should_have_value);
            } else if file.contains("field_null") {
                let optional_model: FieldModelOptional<i32, _> = 
                    FieldModelOptional::new(&data, 0, |buf, off| FieldModelI32::new(buf, off));
                println!("Has value: {}", optional_model.has_value());
                assert_eq!(optional_model.has_value(), should_have_value);
            } else if file.contains("final_value") {
                let optional_model: FinalModelOptional<i32, _> = 
                    FinalModelOptional::new(&data, 0, |buf, off| FinalModelI32::new(buf, off));
                println!("Has value: {}", optional_model.has_value());
                if optional_model.has_value() {
                    let value = optional_model.get(|model| model.get()).unwrap();
                    println!("Value: {}", value);
                    assert_eq!(value, expected_value);
                }
                assert_eq!(optional_model.has_value(), should_have_value);
            } else if file.contains("final_null") {
                let optional_model: FinalModelOptional<i32, _> = 
                    FinalModelOptional::new(&data, 0, |buf, off| FinalModelI32::new(buf, off));
                println!("Has value: {}", optional_model.has_value());
                assert_eq!(optional_model.has_value(), should_have_value);
            }
            println!();
        }
    }

    println!("✅ Rust Optional cross-platform test complete!");
}


