// === 🏛️ Domain Layer: Items Entity ===
// 📖 นี่คือ Entity หลักของระบบ - ข้อมูลหลักที่ไม่ขึ้นกับเทคโนโลยีภายนอก
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    Items entity มีหน้าที่เดียว: เก็บและจัดการข้อมูลหลักของ Item
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม method ใหม่ได้โดยไม่แก้ไข struct เดิม
//    เพิ่ม category ใหม่ได้ใน get_category() โดยไม่แก้ไข logic เดิม
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    Items entity สามารถใช้ในที่ที่ต้องการ domain object ได้
//
// 4️⃣ Interface Segregation Principle (ISP):
//    แต่ละ method มีหน้าที่เฉพาะเจาะจง ไม่บังคับใช้ method ที่ไม่ต้องการ
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    ใช้ IntoTimerHelperShared abstraction แทนการสร้าง timestamp โดยตรง

use chrono::NaiveDateTime;

use crate::{
    models::{
        error::{APIError, IntoErrorResponse},
        item::{Category, Item as ItemModel},
    },
    time_helper::IntoTimerHelperShared,
};

// 📦 Items struct: ข้อมูลหลักของ Item ในระบบ
// - derive Debug: สำหรับการ debug และ print
// - derive Clone: ให้สามารถ copy ข้อมูลได้
// - derive sqlx::FromRow: แปลงข้อมูลจากฐานข้อมูลเป็น struct
// - derive PartialEq: สำหรับการเปรียบเทียบความเท่ากัน
#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct Items{
    pub id: Option<i32>,           // 🆔 Primary key (None สำหรับข้อมูลใหม่)
    pub name: String,              // 📝 ชื่อของ item
    pub category: String,          // 🏷️ ประเภทของ item (เก็บเป็น String)
    pub created_at: NaiveDateTime, // 📅 วันที่สร้าง
    pub updated_at: NaiveDateTime, // 🔄 วันที่อัปเดตล่าสุด
}

impl Items {
    // 🏗️ Constructor: สร้าง Items ใหม่
    // พารามิเตอร์ t เป็น dependency injection สำหรับการจัดการเวลา
    pub fn new(name: String, category: String, t: IntoTimerHelperShared) -> Self{
        Self{
            id: None,                    // 🆔 ยังไม่มี ID (จะได้จากฐานข้อมูลหลังจาก insert)
            name,                        // 📝 ชื่อที่รับมา
            category,                    // 🏷️ ประเภทที่รับมา
            created_at: t.now(),         // 📅 เวลาปัจจุบันจาก timer helper
            updated_at: t.now(),         // 🔄 เวลาปัจจุบันจาก timer helper
        }
    }

    // 🔄 แปลง Entity เป็น Model สำหรับส่งผ่าน API
    // Entity (Domain) -> Model (API Response)
    pub fn to_model(&self) -> Result<ItemModel, Box<dyn IntoErrorResponse>>{
        // 🔍 แปลง String category เป็น Category enum
        let category = match self.get_category(){
            Some(category) => category,
            None => {
                // ❌ ถ้าไม่พบประเภทที่ถูกต้อง ส่ง error กลับ
                return Err(Box::new(APIError::InvalidCategory(self.category.clone())));
            }
        };

        // ✅ สร้าง ItemModel สำหรับ API response
        Ok(ItemModel {
            id: self.id.unwrap(),        // 🆔 ต้องมี ID (unwrap เพราะถ้าเรียก to_model แปลว่ามี ID แล้ว)
            name: self.name.to_string(), // 📝 ชื่อ
            category,                    // 🏷️ ประเภทที่แปลงแล้ว
        })
    }

    // 🏷️ แปลง String category เป็น Category enum
    // เป็น business logic ที่กำหนดว่ามีประเภทไหนบ้างในระบบ
    pub fn get_category(&self) -> Option<Category>{
        match self.category.as_str(){
            "Staff" => Some(Category::Staff),  // 🪄 ไม้เท้าวิเศษ
            "Sword" => Some(Category::Sword),  // ⚔️ ดาบ
            _ => None,                         // ❓ ประเภทที่ไม่รู้จัก
        }
    }
}