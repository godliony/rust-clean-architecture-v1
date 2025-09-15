// === 💾 Infrastructure Layer: Staff Repository ===
// 📖 จัดการการเข้าถึงข้อมูล Staff ในฐานข้อมูล - เป็น concrete implementation
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    StaffRepository มีหน้าที่เดียว: จัดการการเข้าถึงข้อมูล Staff ในฐานข้อมูล
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม method ใหม่ได้โดยไม่แก้ไข interface เดิม
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    StaffRepository สามารถใช้แทน ItemsRepository interface ได้อย่างสมบูรณ์
//
// 4️⃣ Interface Segregation Principle (ISP):
//    Implement เฉพาะ ItemsRepository interface ที่จำเป็น ไม่มีส่วนเกิน
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    ขึ้นอยู่กับ abstraction (ItemsRepository trait) ไม่ใช่ concrete classes

use std::sync::Arc;

use async_trait::async_trait;         // 🔧 สำหรับ async trait
use sqlx::PgPool;                     // 🗄️ PostgreSQL connection pool
use tracing::error;                   // 📝 Logging สำหรับ error

use crate::entities::items::Items;    // 🏛️ Domain Entity

use super::items::{ItemsRepository, SharedItemsRepository}; // 📋 Interface ที่ต้อง implement

// 📦 StaffRepository struct: จัดการข้อมูล Staff ในฐานข้อมูล
pub struct StaffRepository {
    db_pool: PgPool,                  // 🗄️ Database connection pool สำหรับ PostgreSQL
}

impl StaffRepository {
    // 🏗️ Factory method: สร้าง StaffRepository และ wrap ด้วย Arc สำหรับ shared ownership
    pub fn creation(db_pool: PgPool) -> SharedItemsRepository {
        Arc::new(Self {db_pool})            // 🔗 Arc สำหรับให้หลาย ๆ ที่ใช้ร่วมกันได้
    }
}

// 🔧 ImplementItemsRepository trait สำหรับ StaffRepository
// async_trait เพราะ Rust ยังไม่รองรับ async fn ใน trait ธรรมดา
#[async_trait]
impl ItemsRepository for StaffRepository {
    // 🔍 ค้นหา Staff ตามชื่อ
    async fn find_by_name(&self, name: String) -> Result<Items, sqlx::Error>{
        let item = match sqlx::query_as::<_, Items>(
            "SELECT * FROM items WHERE (name = $1 AND category = 'Staff');", // 📊 SQL query
        )
        .bind(name.clone())               // 🔗 Bind parameter เพื่อป้องกัน SQL injection
        .fetch_one(&self.db_pool)         // 🎯 ดึงข้อมูลแถวเดียว
        .await
        {
            Ok(item) => item,             // ✅ สำเร็จ
            Err(e) => {
                error!("Failed to find item by name: {}", e); // 📝 Log error
                return Err(e);            // ❌ ส่ง error กลับ
            }
        };
        Ok(item)
    }

    // ➕ เพิ่ม Staff ใหม่ลงฐานข้อมูล และส่งกลับ ID ของข้อมูลที่เพิ่ม
    async fn insert(&self, item: Items) -> Result<i32, sqlx::Error>{
        let item = match sqlx::query_as::<_,Items>(
            "INSERT INTO items (name, category, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(item.name)                  // 🔗 Bind parameter: ป้องกัน SQL injection
        .bind(item.category)              // 🔗 Bind parameter: ประเภท
        .bind(item.created_at)            // 🔗 Bind parameter: วันที่สร้าง
        .bind(item.updated_at)            // 🔗 Bind parameter: วันที่อัปเดต
        .fetch_one(&self.db_pool)         // 🎯 ดึงข้อมูลแถวเดียวที่เพิ่งเพิ่ม
        .await
        {
            Ok(item) => item,             // ✅ สำเร็จ
            Err(e) => {
                error!("Failed to insert item: {:?}",e); // 📝 Log error
                return Err(e);            // ❌ ส่ง error กลับ
            }
        };
        // 🔍 ตรวจสอบว่าได้ ID กลับมาหรือไม่
        Ok(match item.id {
            Some(id) => id,               // ✅ ได้ ID แล้ว
            None => {
                error!("Failed to insert item: id is missing"); // 📝 Log error
                return Err(sqlx::Error::RowNotFound); // ❌ ไม่ได้ ID (ไม่น่าเกิดขึ้น)
            }
        })
    }

    // 🔍 ค้นหา Staff ตาม ID
    async fn find_by_id(&self, id: i32) -> Result<Items, sqlx::Error>{
        let item = match sqlx::query_as::<_,Items>(
            "SELECT * FROM items WHERE (id = $1 AND category = 'Staff');", // 📊 SQL query
        )
        .bind(id)                         // 🔗 Bind parameter: ID ที่ต้องการหา
        .fetch_one(&self.db_pool)         // 🎯 ดึงข้อมูลแถวเดียว
        .await
        {
            Ok(item) => item,             // ✅ สำเร็จ
            Err(e) => {
                error!("Failed to find item by id: {}",e); // 📝 Log error
                return Err(e);            // ❌ ส่ง error กลับ
            }
        };
        Ok(item)
    }

}