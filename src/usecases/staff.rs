// === 🧠 Application Layer: Staff Use Case ===
// 📖 ตรรกะทางธุรกิจสำหรับการจัดการ Staff - orchestrates ระหว่าง layers
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    StaffUsecase มีหน้าที่เดียว: จัดการ business logic ของ Staff
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม use case ใหม่ได้โดยไม่แก้ไข interface หรือ repository
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    StaffUsecase ทำงานกับ interface abstractions ที่สามารถถูกแทนที่ได้
//
// 4️⃣ Interface Segregation Principle (ISP):
//    ใช้เฉพาะ method ที่จำเป็นจาก ItemsRepository และ TimerHelper
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    ขึ้นอยู่กับ abstractions: SharedItemsRepository และ IntoTimerHelperShared
//    ไม่ขึ้นอยู่กับ concrete implementations

use std::sync::Arc;

use crate::{
    models::{
        error::{APIError, IntoErrorResponse},  // 📋 Error handling models
        item::{Item, StaffAdding},             // 📋 API models
    },
    repositories::items::SharedItemsRepository, // 💾 Repository interface
    time_helper::IntoTimerHelperShared,        // ⏰ Time utility
};

// 📦 StaffUsecase struct: จัดการ business logic ของ Staff
pub struct StaffUsecase {
    items_repository: SharedItemsRepository,   // 💾 Dependency: Repository สำหรับเข้าถึงข้อมูล
    timer_helper: IntoTimerHelperShared,       // ⏰ Dependency: เครื่องมือจัดการเวลา
}

impl StaffUsecase {
    // 🏗️ Factory method: สร้าง StaffUsecase พร้อม dependencies
    pub fn creation(
        items_repository: SharedItemsRepository,   // 💾 Repository dependency
        timer_helper: IntoTimerHelperShared,       // ⏰ Timer dependency
    ) -> Arc<Self> {
        Arc::new(Self {
            items_repository,
            timer_helper,
        })
    }

    // ➕ Business Logic: เพิ่ม Staff ใหม่
    // นี่คือหัวใจของ Use Case - จัดการ business rules และ flow
    pub async fn adding(&self, staff: StaffAdding) -> Result<Item, Box<dyn IntoErrorResponse>> {
        // 🔍 ขั้นตอนที่ 1: ตรวจสอบว่ามี Staff ชื่อนี้อยู่แล้วหรือไม่ (Business Rule)
        if let Ok(_) = self.items_repository.find_by_name(staff.name.clone()).await {
            return Err(Box::new(APIError::ItemAlreadyExists(staff.name.clone())));
        };

        // ➕ ขั้นตอนที่ 2: แปลง Model เป็ Entity และบันทึกลงฐานข้อมูล

        // 🔄 แบบเดิม: สร้าง Entity โดยตรง (ไม่ตาม Clean Architecture flow)
        // let id = match self
        //     .items_repository
        //     .insert(ItemsEntity::new(                    // 🏗️ สร้าง Entity ใหม่
        //         staff.name.clone(),
        //         Category::Staff.to_string(),             // 🏷️ กำหนดประเภทเป็น Staff
        //         Arc::clone(&self.timer_helper),          // ⏰ ใช้ timer helper สำหรับ timestamp
        //     ))
        //     .await

        // ✅ แบบใหม่: ใช้ Model → Entity → Database flow ตาม Clean Architecture
        let id = match self
            .items_repository
            .insert(staff.to_entity(Arc::clone(&self.timer_helper))) // 🔄 ใช้ to_entity() แปลง Model → Entity
            .await
        {
            Ok(id) => id,                                // ✅ ได้ ID ใหม่จากฐานข้อมูล
            Err(e) => return Err(Box::new(APIError::AddingItemError(e))), // ❌ Error ในการบันทึก
        };

        // 🔍 ขั้นตอนที่ 3: ดึงข้อมูลที่เพิ่งบันทึกกลับมาเพื่อยืนยัน
        let staff_entity = match self.items_repository.find_by_id(id).await {
            Ok(r) => r,                                  // ✅ พบข้อมูล
            Err(_) => return Err(Box::new(APIError::ItemNotFound(id))), // ❌ ไม่พบข้อมูล (ไม่น่าเกิดขึ้น)
        };

        // 🔄 ขั้นตอนที่ 4: แปลง Entity เป็น Model สำหรับ API response
        Ok(match staff_entity.to_model(){
            Ok(r) => r,                                  // ✅ แปลงสำเร็จ
            Err(e) => return Err(e),                     // ❌ Error ในการแปลง
        })
    }
}
