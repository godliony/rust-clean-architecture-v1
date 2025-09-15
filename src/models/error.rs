// === ❌ Error Handling Models ===
// 📖 จัดการ Error แบบมีระบบและแปลงเป็น HTTP Response
// 🎯 SOLID Principles: Single Responsibility & Interface Segregation

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// 📋 ErrorResponse: โครงสร้างมาตรฐานสำหรับ HTTP Error Response
// 🎯 SOLID: Single Responsibility - เฉพาะจัดการ error response
#[derive(Debug)]
pub struct ErrorResponse {
    pub error: String,          // 💬 ข้อความ error
    pub status_code: StatusCode, // 🔢 HTTP status code
}

// 🔄 แปลง ErrorResponse เป็น HTTP Response
// 🎯 SOLID: Interface Segregation - implement เฉพาะที่จำเป็น
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!(
                {
                    "error": self.error    // 📤 JSON format: {"error": "message"}
                }
            )),
        )
            .into_response()
    }
}

// 📋 Trait สำหรับแปลง Error เป็น ErrorResponse
// 🎯 SOLID: Interface Segregation - แยก interface สำหรับแต่ละหน้าที่
pub trait IntoErrorResponse {
    fn error(&self) -> ErrorResponse;
}

// ❌ APIError Enum: กำหนดประเภท Error ที่เป็นไปได้ในระบบ
// 🎯 SOLID: Open/Closed Principle - เพิ่ม error type ใหม่ได้โดยไม่แก้โค้ดเดิม
pub enum APIError {
    InvalidCategory(String),      // 🏷️ ประเภทที่ไม่ถูกต้อง
    ItemAlreadyExists(String),    // 🔄 Item ที่มีอยู่แล้ว
    AddingItemError(sqlx::Error), // 💾 Error จากฐานข้อมูลขณะเพิ่มข้อมูล
    ItemNotFound(i32),           // 🔍 ไม่พบ Item ที่ต้องการ
}

// 🔄 Implementation ของ IntoErrorResponse trait สำหรับ APIError
// 🎯 SOLID: Liskov Substitution - APIError สามารถใช้แทน IntoErrorResponse ได้
impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            // 🏷️ ประเภทไม่ถูกต้อง - เป็น client error
            Self::InvalidCategory(category) => ErrorResponse {
                error: format!("Invalid category: {}", category),
                status_code: StatusCode::BAD_REQUEST,
            },
            // 🔄 Item มีอยู่แล้ว - เป็น conflict error
            Self::ItemAlreadyExists(name) => ErrorResponse {
                error: format!("Item is already exists: {}", name),
                status_code: StatusCode::CONFLICT,
            },
            // 💾 Error จากฐานข้อมูล - เป็น server error
            Self::AddingItemError(err) => ErrorResponse {
                error: format!("Failed to add item {:?}", err),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            // 🔍 ไม่พบข้อมูล - เป็น not found error
            Self::ItemNotFound(id) => ErrorResponse {
                error: format!("Item not found: {}", id),
                status_code: StatusCode::NOT_FOUND,
            },
        }
    }
}
