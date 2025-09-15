// === 📋 Repository Interface & Mock Generation ===
// 📖 กำหนด Interface และสร้าง Mock สำหรับการทดสอบ
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    ItemsRepository trait มีหน้าที่เดียว: กำหนด interface สำหรับการเข้าถึงข้อมูล Items
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม method ใหม่ได้โดยไม่แก้ไข interface เดิม
//    สามารถสร้าง implementation ใหม่ได้โดยไม่แก้ไข trait
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    MockItemsRepository และ concrete implementation สามารถใช้แทนกันได้
//
// 4️⃣ Interface Segregation Principle (ISP):
//    แยก interface เฉพาะสำหรับ Items ไม่รวมกับ interface อื่น
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    High-level modules สามารถขึ้นอยู่กับ trait นี้แทน concrete classes

// 🔧 นำเข้า Arc (Atomic Reference Counted) จาก standard library
// Arc เป็นตัว smart pointer ที่ใช้แชร์ข้อมูลระหว่าง threads ได้อย่างปลอดภัย
use std::sync::Arc;
use async_trait::async_trait;    // 🔧 สำหรับ async functions ใน traits
use mockall::automock;           // 🎭 สร้าง Mock objects อัตโนมัติ

use crate::entities::items::Items;

// 🔗 Type alias สำหรับ shared Repository
// dyn = dynamic dispatch, Send + Sync = thread-safe
pub type SharedItemsRepository = Arc<dyn ItemsRepository + Send + Sync>;

// 📋 Repository Trait: Interface สำหรับการเข้าถึงข้อมูล Items
//
// 🎭 #[automock]: สร้าง MockItemsRepository อัตโนมัติ
//    - จะสร้าง struct MockItemsRepository ให้เราใช้ในการทดสอบ
//    - ไม่ต้องเขียน Mock เอง ช่วยประหยัดเวลาและลดความผิดพลาด
//
// 🔧 #[async_trait]: ใช้เพราะ Rust ยังไม่รองรับ async fn ใน trait ธรรมดา
//    - แปลง async fn ให้เป็น Future และจัดการ lifetime ให้เรา
#[async_trait]
#[automock]
pub trait ItemsRepository {
    // 🔍 ค้นหา Item ตามชื่อ
    async fn find_by_name(&self, name: String) -> Result<Items, sqlx::Error>;

    // ➕ เพิ่ม Item ใหม่ ส่งกลับ ID ของข้อมูลที่เพิ่ม
    async fn insert(&self, item: Items) -> Result<i32, sqlx::Error>;

    // 🔍 ค้นหา Item ตาม ID
    async fn find_by_id(&self, id: i32) -> Result<Items, sqlx::Error>;
}
