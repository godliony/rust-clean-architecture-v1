// === 🌐 Interface Layer: Staff Handler ===
// 📖 จัดการ HTTP requests และ responses สำหรับ Staff - เป็นจุดติดต่อกับโลกภายนอก
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    staff_adding function มีหน้าที่เดียว: จัดการ HTTP request/response สำหรับการเพิ่ม Staff
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม handler function ใหม่ได้โดยไม่แก้ไข function เดิม
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    Handler ทำงานกับ StaffUsecase interface ที่สามารถถูกแทนที่ได้
//
// 4️⃣ Interface Segregation Principle (ISP):
//    ใช้เฉพาะ method ที่จำเป็นจาก StaffUsecase
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    ขึ้นอยู่กับ abstraction (Arc<StaffUsecase>) ไม่ใช่ concrete implementation

use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json}; // 🌐 Axum web framework

use crate::{
    models::item::StaffAdding,          // 📋 Input model สำหรับ API
    usecases::staff::StaffUsecase       // 🧠 Use case สำหรับ business logic
};

// 📮 HTTP POST handler: เพิ่ม Staff ใหม่
// รับ JSON input และส่ง JSON response กลับ
pub async fn staff_adding(
    Json(body): Json<StaffAdding>,      // 📥 ดึงข้อมูล JSON จาก request body
    staff_usecase: Arc<StaffUsecase>,   // 🧠 Dependency injection: Use case
) ->impl IntoResponse {
    // 🔄 เรียก Use Case เพื่อทำ business logic
    let staff = match staff_usecase.adding(body).await{
        Ok(r) => r,                                      // ✅ สำเร็จ: ได้ข้อมูล Staff ใหม่
        Err(e) => return e.error().into_response(),      // ❌ ผิดพลาด: แปลง error เป็น HTTP response
    };

    // 📤 ส่ง response กลับ: HTTP 201 Created พร้อมข้อมูล Staff
    (StatusCode::CREATED, Json(staff)).into_response()
}