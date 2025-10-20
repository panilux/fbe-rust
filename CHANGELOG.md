# Changelog

All notable changes to this project will be documented in this file.

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

