// === 📋 API Models Layer ===
// 📖 Data Transfer Objects (DTOs) สำหรับการสื่อสารกับ API
// 🎯 SOLID Principle: Single Responsibility - แต่ละ struct มีหน้าที่เฉพาะเจาะจง

use crate::{entities::items::Items as ItemsEntity, time_helper::IntoTimerHelperShared};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// 🏷️ Category Enum: กำหนดประเภทของ Item ที่รองรับในระบบ
//
// 📋 Serialize/Deserialize: แปลงข้อมูลระหว่าง Rust struct และ JSON
// 🔄 Clone: ให้สามารถ copy ข้อมูลได้
// ⚖️ PartialEq: สำหรับการเปรียบเทียบ
//
// 🎯 SOLID: Open/Closed Principle - เพิ่มประเภทใหม่ได้โดยไม่แก้โค้ดเดิม
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Category {
    Staff,  // 🪄 ไม้เท้าวิเศษ
    Sword,  // ⚔️ ดาบ
}

// 🎨 Display Trait Implementation: แปลง Category เป็น String
// 🎯 SOLID: Interface Segregation - implement เฉพาะ trait ที่จำเป็น
impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Staff => write!(f, "Staff"),  // 🪄 แปลง Staff enum เป็น "Staff" string
            Self::Sword => write!(f, "Sword"),  // ⚔️ แปลง Sword enum เป็น "Sword" string
        }
    }
}

// 📦 Item Model: API response model สำหรับ Item ที่สมบูรณ์
// 🎯 SOLID: Single Responsibility - เฉพาะข้อมูลสำหรับ API response
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Item {
    pub id: i32,           // 🆔 Primary key จากฐานข้อมูล
    pub name: String,      // 📝 ชื่อของ item
    pub category: Category, // 🏷️ ประเภทในรูปแบบ enum
}

impl Item {
    // 🔄 แปลง API Model เป็น Domain Entity
    // 🎯 SOLID: Dependency Inversion - รับ timer helper เป็น parameter
    pub fn to_entity(&self, t: IntoTimerHelperShared) -> ItemsEntity {
        ItemsEntity::new(self.name.to_string(), self.category.to_string(), t)
    }
}

// 📥 StaffAdding Model: ข้อมูลสำหรับการเพิ่ม Staff ใหม่
// 🎯 SOLID: Single Responsibility - เฉพาะข้อมูลที่จำเป็นสำหรับการเพิ่ม Staff

// 🔄 แบบเดิม: ไม่มี category field
// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub struct StaffAdding {
//     pub name: String,      // 📝 ชื่อของ Staff ที่จะเพิ่ม
// }

// ✅ แบบใหม่: เพิ่ม category field เพื่อให้ Client ระบุประเภทได้เองตาม Clean Architecture
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct StaffAdding {
    pub name: String,      // 📝 ชื่อของ Staff ที่จะเพิ่ม
    pub category: Category, // 🏷️ ประเภทของ Item
}

impl StaffAdding {
    // 🔄 แปลง StaffAdding Model เป็น Domain Entity
    // ✅ ใช้ to_entity() method เพื่อให้เป็นไปตาม Client → Model → Entity → Database flow
    pub fn to_entity(&self, t: IntoTimerHelperShared) -> ItemsEntity {
        ItemsEntity::new(self.name.to_string(), self.category.to_string(), t)
    }
}
