/*!
 * FBE Struct Inheritance Support for Rust
 * HERSEY DAHA IYI BIR PANILUX ICIN! 🚀
 */

use crate::buffer::{WriteBuffer, ReadBuffer};

// Base struct: Person
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
        }
    }
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        let mut offset = 0;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_i32(offset, self.age);
        offset += 4;
        buffer.set_size(offset);  // Update buffer size
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        let mut offset = 0;
        let name = buffer.read_string(offset);
        offset += 4 + name.len();
        let age = buffer.read_i32(offset);
        Self { name, age }
    }
}

// Derived struct: Employee (inherits from Person)
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    // Base fields (from Person)
    pub name: String,
    pub age: i32,
    // Derived fields
    pub company: String,
    pub salary: f64,
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
            company: String::new(),
            salary: 0.0,
        }
    }
}

impl Employee {
    pub fn new(name: String, age: i32, company: String, salary: f64) -> Self {
        Self { name, age, company, salary }
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        // Serialize base fields first
        let mut offset = 0;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_i32(offset, self.age);
        offset += 4;
        // Then serialize derived fields
        buffer.write_string(offset, &self.company);
        offset += 4 + self.company.len();
        buffer.write_f64(offset, self.salary);
        offset += 8;
        buffer.set_size(offset);  // Update buffer size
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        // Deserialize base fields first
        let mut offset = 0;
        let name = buffer.read_string(offset);
        offset += 4 + name.len();
        let age = buffer.read_i32(offset);
        offset += 4;
        // Then deserialize derived fields
        let company = buffer.read_string(offset);
        offset += 4 + company.len();
        let salary = buffer.read_f64(offset);
        Self { name, age, company, salary }
    }
}

// Multi-level inheritance: Manager (inherits from Employee)
#[derive(Debug, Clone, PartialEq)]
pub struct Manager {
    // Base fields (from Person)
    pub name: String,
    pub age: i32,
    // Employee fields
    pub company: String,
    pub salary: f64,
    // Manager fields
    pub team_size: i32,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
            company: String::new(),
            salary: 0.0,
            team_size: 0,
        }
    }
}

impl Manager {
    pub fn new(name: String, age: i32, company: String, salary: f64, team_size: i32) -> Self {
        Self { name, age, company, salary, team_size }
    }

    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {
        // Serialize all base fields in order
        let mut offset = 0;
        buffer.write_string(offset, &self.name);
        offset += 4 + self.name.len();
        buffer.write_i32(offset, self.age);
        offset += 4;
        buffer.write_string(offset, &self.company);
        offset += 4 + self.company.len();
        buffer.write_f64(offset, self.salary);
        offset += 8;
        buffer.write_i32(offset, self.team_size);
        offset += 4;
        buffer.set_size(offset);  // Update buffer size
        offset
    }

    pub fn deserialize(buffer: &ReadBuffer) -> Self {
        // Deserialize all base fields in order
        let mut offset = 0;
        let name = buffer.read_string(offset);
        offset += 4 + name.len();
        let age = buffer.read_i32(offset);
        offset += 4;
        let company = buffer.read_string(offset);
        offset += 4 + company.len();
        let salary = buffer.read_f64(offset);
        offset += 8;
        let team_size = buffer.read_i32(offset);
        Self { name, age, company, salary, team_size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_serialization() {
        let person = Person::new("Alice".to_string(), 30);
        let mut buffer = WriteBuffer::new();
        buffer.reserve(100);  // Reserve space
        let size = person.serialize(&mut buffer);
        assert_eq!(size, 13); // 4 (len) + 5 (Alice) + 4 (age)

        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let person2 = Person::deserialize(&read_buffer);
        assert_eq!(person, person2);
    }

    #[test]
    fn test_employee_serialization() {
        let employee = Employee::new("Bob".to_string(), 35, "Panilux".to_string(), 75000.50);
        let mut buffer = WriteBuffer::new();
        buffer.reserve(100);  // Reserve space
        let size = employee.serialize(&mut buffer);
        assert_eq!(size, 30); // 4+3 + 4 + 4+7 + 8

        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let employee2 = Employee::deserialize(&read_buffer);
        assert_eq!(employee, employee2);
    }

    #[test]
    fn test_manager_serialization() {
        let manager = Manager::new("Charlie".to_string(), 40, "Panilux".to_string(), 95000.75, 10);
        let mut buffer = WriteBuffer::new();
        buffer.reserve(100);  // Reserve space
        let size = manager.serialize(&mut buffer);
        assert_eq!(size, 38); // 4+7 + 4 + 4+7 + 8 + 4

        let mut read_buffer = ReadBuffer::new();
        read_buffer.attach_buffer(buffer.data(), 0, buffer.data().len());
        let manager2 = Manager::deserialize(&read_buffer);
        assert_eq!(manager, manager2);
    }
}

