// === 🚀 Entry Point ของระบบ Clean Architecture ===
// 📖 ไฟล์นี้เป็นจุดเริ่มต้นของระบบ - อ่านไฟล์นี้ก่อนเพื่อเข้าใจโฟลว์ทั้งหมด
//
// 🎯 SOLID Principles ที่ใช้ในไฟล์นี้:
//
// 1️⃣ Single Responsibility Principle (SRP):
//    main function มีหน้าที่เดียว: ตั้งค่าและเริ่มต้นระบบ
//
// 2️⃣ Open/Closed Principle (OCP):
//    สามารถเพิ่ม route และ middleware ใหม่ได้โดยไม่แก้ไข main function
//
// 3️⃣ Liskov Substitution Principle (LSP):
//    สามารถแทนที่ implementation ต่าง ๆ ได้ผ่าน dependency injection
//
// 4️⃣ Interface Segregation Principle (ISP):
//    ใช้เฉพาะ interface ที่จำเป็นสำหรับแต่ละ component
//
// 5️⃣ Dependency Inversion Principle (DIP):
//    การสร้าง dependencies จากภายนอกและ inject เข้าไป
//    ใช้ Arc สำหรับ shared ownership ของ dependencies

use axum::{Router, http::Method, routing::post};
use rust_clean_architecture_v1::{
    database,                                          // 🗄️ จัดการการเชื่อมต่อฐานข้อมูล
    handlers::staff::staff_adding,                     // 🌐 Handler - จัดการ HTTP requests
    repositories::staff::StaffRepository,              // 💾 Repository - เข้าถึงฐานข้อมูล
    setting::Setting,                                  // ⚙️ การตั้งค่าระบบ
    time_helper::TimerHelper,                          // ⏰ เครื่องมือจัดการเวลา
    usecases::staff::StaffUsecase,                     // 🧠 Use Case - ตรรกะทางธุรกิจ
};
use std::{net::SocketAddr, sync::Arc};               // 🔧 Standard library tools
use tokio::net::TcpListener;                          // 🌐 Async TCP listener
use tower_http::{                                     // 🔌 HTTP middleware
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;                                    // 📝 Logging

#[tokio::main]
async fn main() {
    // 📝 ขั้นตอนที่ 1: ตั้งค่า logging สำหรับการ debug
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // ⚙️ ขั้นตอนที่ 2: โหลดการตั้งค่าระบบจากไฟล์ config
    let setting = Setting::new().unwrap();
    info!("setting has been loaded");

    // 🗄️ ขั้นตอนที่ 3: เชื่อมต่อฐานข้อมูล (Infrastructure Layer)
    let db_pool = database::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    // 🏗️ ขั้นตอนที่ 4: สร้าง Dependencies ตาม Clean Architecture
    // สร้างตามลำดับ: Repository -> Helper -> Use Case
    let staff_repository = StaffRepository::creation(db_pool.clone());  // 💾 Infrastructure Layer
    let timer_helper = TimerHelper::Directly.creation();                // ⏰ Utility
    let staff_usecase =                                                  // 🧠 Application Layer
        StaffUsecase::creation(Arc::clone(&staff_repository), Arc::clone(&timer_helper));

    // 🌐 ขั้นตอนที่ 5: สร้าง Web Router และ Middleware (Interface Layer)
    let app = Router::new()
        .layer(
            CorsLayer::new()                                            // 🔒 CORS สำหรับ Cross-Origin requests
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .route(
            "/items/staff",                                             // 🛣️ Route definition
            post({                                                      // 📮 HTTP POST handler
                let usecase = Arc::clone(&staff_usecase);               // 🔗 Dependency injection
                move |body| staff_adding(body, usecase)                 // 🌐 Handler function
            }),
        )
        .layer(TraceLayer::new_for_http());                            // 📊 HTTP tracing middleware

    // 🚀 ขั้นตอนที่ 6: เริ่มต้น Server
    let addr = SocketAddr::from(([0, 0, 0, 0], setting.server.port as u16));

    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server running on port {}", setting.server.port);
    axum::serve(listener, app).await.unwrap();
}
