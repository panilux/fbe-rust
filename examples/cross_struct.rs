use fbe::buffer::{ReadBuffer, WriteBuffer};
use std::fs;

mod side {
    #[repr(i8)]
    #[derive(Debug, Clone, Copy, Default)]
    pub enum Side {
        #[default]
        Buy = 0,
        Sell = 1,
    }
}

mod user {
    use super::side::Side;
    use fbe::buffer::{ReadBuffer, WriteBuffer};

    #[derive(Debug, Clone, Default)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub side: Side,
    }

    impl User {
        pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
            let total_size = 4 + 4 + self.name.len() + 1;
            buffer.allocate(total_size);

            let mut offset = 0;
            buffer.write_i32(offset, self.id);
            offset += 4;
            buffer.write_string(offset, &self.name);
            offset += 4 + self.name.len();
            buffer.write_i8(offset, self.side as i8);
            offset += 1;
            offset
        }

        pub fn deserialize(buffer: &ReadBuffer) -> Self {
            let mut offset = 0;
            Self {
                id: {
                    let val = buffer.read_i32(offset);
                    offset += 4;
                    val
                },
                name: {
                    let val = buffer.read_string(offset);
                    offset += 4 + val.len();
                    val
                },
                side: unsafe { std::mem::transmute(buffer.read_i8(offset)) },
            }
        }
    }
}

use side::Side;
use user::User;

fn main() {
    println!("Testing cross-platform struct serialization...\n");

    // Test 1: Rust → PHP
    println!("1. Rust → PHP");
    let user = User {
        id: 42,
        name: "Panilux".to_string(),
        side: Side::Buy,
    };

    let mut buffer = WriteBuffer::new();
    user.serialize(&mut buffer);

    let rust_binary: String = buffer.data().iter().map(|b| format!("{:02x}", b)).collect();
    println!("   Rust binary: {}", rust_binary);

    // Write to file for PHP to read
    fs::write("/tmp/rust_struct_to_php.bin", buffer.data()).unwrap();
    println!("   Saved to /tmp/rust_struct_to_php.bin");

    // Test 2: Read PHP binary
    println!("\n2. PHP → Rust");
    if let Ok(php_data) = fs::read("/tmp/php_struct_to_rust.bin") {
        let php_hex: String = php_data.iter().map(|b| format!("{:02x}", b)).collect();
        println!("   PHP binary: {}", php_hex);

        let reader = ReadBuffer::from(php_data);
        let user2 = User::deserialize(&reader);

        println!("   Deserialized from PHP:");
        println!("     id: {}", user2.id);
        println!("     name: {}", user2.name);
        println!("     side: {:?}", user2.side);

        println!("   ✅ PHP → Rust successful!");
    } else {
        println!("   ⏳ Waiting for PHP binary...");
    }

    println!("\n✅ Cross-platform struct test completed!");
}
