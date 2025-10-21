# Changelog

All notable changes to this project will be documented in this file.

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
- Fixed cross_test.rs to use correct buffer API methods (data() instead of buffer())

### Verified
- ✅ Rust → Rust round-trip serialization
- ✅ PHP → PHP round-trip serialization
- ✅ Rust → PHP cross-platform binary compatibility
- ✅ PHP → Rust cross-platform binary compatibility

## [0.0.1] - 2025-10-20

### Added
- Initial Rust FBE implementation
- WriteBuffer and ReadBuffer structs
- FieldModel traits
- Basic type support (primitives)
- Rust code generator (fbec-rust)

