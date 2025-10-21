## [0.1.0] - 2025-10-21

### Added
- **Binary Compatibility Tests:** Cross-platform PHP ↔ Rust verification
- **THE CHALLENGE:** Passed! %100 binary compatible with PHP implementation
- **hex crate:** For binary debugging

### Verified
- ✅ Rust → Rust: PASS
- ✅ PHP → Rust: PASS (reads PHP binary)
- ✅ Rust → PHP: PASS (PHP reads Rust binary)
- ✅ Binary format identical (byte-for-byte)

## [0.0.9] - 2025-10-21

### Added
- **Float/Double Collection Support:** VectorF32, VectorF64, ArrayF32, ArrayF64
- **Buffer Methods:** write_vector_f32, read_vector_f32, write_vector_f64, read_vector_f64
- **Buffer Methods:** write_array_f32, read_array_f32, write_array_f64, read_array_f64

### Verified
- ✅ Float collections working
- ✅ Double collections working
- ✅ Zero-cost abstractions maintained

# Changelog

All notable changes to this project will be documented in this file.

## [0.0.8] - 2025-10-21

### Added
- **String Collection Support:** VectorString, ArrayString
- **Buffer Methods:** write_vector_string, read_vector_string, write_array_string, read_array_string
- **Field Models:** FieldModelVectorString, FieldModelArrayString (with mut variants)
- **extra() implementation:** Dynamic size calculation for variable-length strings

### Verified
- ✅ String collections working
- ✅ Variable-size string handling
- ✅ Zero-cost abstractions maintained

## [0.0.7] - 2025-10-21

### Added
- **Collection Field Models:** Vector, Array, Map, Set
  - FieldModelVectorI32: Dynamic arrays with pointer-based storage
  - FieldModelArrayI32: Fixed-size arrays with inline storage
  - FieldModelMapI32: Key-value pairs (HashMap integration)
  - FieldModelSetI32: Unique values (HashSet integration)
- **Comprehensive Test Suite:** test_field_model_collections.rs
- **extra() method:** Calculates dynamic collection sizes from buffer

### Verified
- ✅ All 4 collection field models working
- ✅ HashMap/HashSet integration
- ✅ Zero-cost abstractions maintained

## [0.0.6] - 2025-10-21

### Added
- **Complete FieldModel Library:** All primitive and complex type field models
  - Primitives: Bool, I8-64, U8-64, F32, F64
  - Complex: String, Timestamp, UUID, Bytes, Decimal
- **Macro-based Implementation:** Zero-cost abstractions for primitive types
- **Comprehensive Test Suite:** test_field_model.rs testing all field models
- **Modern Rust Patterns:** Trait-based, lifetime-safe implementations

### Verified
- ✅ All field models working correctly
- ✅ Zero-cost abstractions (compile-time optimizations)
- ✅ Lifetime-safe buffer references

## [0.0.5] - 2025-10-21

### Added
- **From<Vec<u8>> trait for ReadBuffer:** Convenient constructor from byte vector
- **Cross-platform struct example:** cross_struct.rs demonstrating PHP ↔ Rust compatibility

### Improved
- Better ReadBuffer ergonomics with From trait
- Cross-platform struct serialization verified

### Verified
- ✅ Rust → PHP: Binary identical
- ✅ PHP → Rust: Binary identical

## [0.0.4] - 2025-10-21

### Added
- **vector<T>** collection support (dynamic arrays with pointer-based storage)
- **array[N]** collection support (fixed-size inline arrays)
- **map<K,V>** collection support (key-value pairs)
- **set<T>** collection support (unique values, same format as vector)
- Individual collection tests for each type
- Cross-platform vector test (Rust ↔ PHP)

### Implemented
- `write_vector_i32()` / `read_vector_i32()` for dynamic arrays
- `write_array_i32()` / `read_array_i32()` for fixed-size arrays
- `write_map_i32()` / `read_map_i32()` for key-value maps
- `write_set_i32()` / `read_set_i32()` for unique value sets

### Verified
- ✅ All collections working in Rust
- ✅ Cross-platform binary compatibility for individual collections
- ✅ Vector cross-platform test passed

### Note
- Combined collection tests require struct-based serialization pattern
- Current implementation supports i32 types, extensible to other types

## [0.0.3] - 2025-10-21

### Added
- **timestamp** type support (uint64, nanoseconds since epoch)
- **uuid** type support (16 bytes, standard UUID format)
- **bytes** type support (size-prefixed binary data)
- **decimal** type support (16 bytes, .NET Decimal format)
- Comprehensive type tests
- Cross-platform type tests (Rust ↔ PHP)

### Verified
- ✅ All new types working in Rust
- ✅ Cross-platform binary compatibility with PHP
- ✅ Round-trip serialization for all types

## [0.0.2] - 2025-10-21

### Added
- `write_string()` method to WriteBuffer for string serialization
- `read_string()` method to ReadBuffer for string deserialization
- Cross-platform test example that reads/writes PHP binaries
- Test modules for User struct and Side enum

### Fixed
- User::serialize() now properly allocates buffer before writing
- Side enum now has Default derive trait
- Enum serialization uses i8 instead of i32 for correct binary format

### Verified
- ✅ Cross-platform serialization working (PHP ↔ Rust)
- ✅ Binary format matches between implementations

## [0.0.1] - 2025-10-20

### Added
- Initial FBE Rust implementation
- WriteBuffer with basic types (bool, int8-64, uint8-64, float, double)
- ReadBuffer with basic types
- Basic test suite

HERSEY DAHA IYI BIR PANILUX ICIN! 🚀

## [0.1.1] - 2025-10-21

### Added
- Optional type support (Type?)
- write_optional_i32(), write_optional_string(), write_optional_f64()
- read_optional_i32(), read_optional_string(), read_optional_f64()
- has_value() method for null checking

### Format
- 1 byte: has_value flag (0=null, 1=has value)
- 4 bytes: pointer to data
- At pointer: actual value

## [0.1.2] - 2025-10-21

### Added
- Nested struct support (Address, UserWithAddress)
- FBE-compliant struct serialization pattern
- Comprehensive FBE ground truth validation

### Validated
- Rust → FBE Python: PASS
- PHP → FBE Python: PASS
- Rust ↔ PHP: Binary identical (233 bytes)
