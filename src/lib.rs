// === 🏗️ Clean Architecture Library Structure ===
// 📖 คู่มือการอ่านโค้ด: เริ่มต้นจากไฟล์นี้เพื่อเข้าใจโครงสร้างทั้งหมด

// 1️⃣ เริ่มอ่านจาก main.rs เพื่อเข้าใจ entry point
// 2️⃣ อ่าน entities - ข้อมูลหลักของระบบ (Domain Layer)
// 3️⃣ อ่าน repositories - การเข้าถึงข้อมูล (Infrastructure Layer)
// 4️⃣ อ่าน usecases - ตรรกะทางธุรกิจ (Application Layer)
// 5️⃣ อ่าน handlers - จัดการ HTTP requests (Interface Layer)
// 6️⃣ อ่าน models - โครงสร้างข้อมูลสำหรับ API
// 7️⃣ อ่าง setting, database, time_helper - เครื่องมือช่วยเหลือ

pub mod database;      // 🗄️  การเชื่อมต่อฐานข้อมูล
pub mod entities;      // 🏛️  Domain Layer - ข้อมูลหลักของระบบ
pub mod handlers;      // 🌐  Interface Layer - จัดการ HTTP requests/responses
pub mod models;        // 📋  Data Transfer Objects - โครงสร้างข้อมูลสำหรับ API
pub mod repositories;  // 💾  Infrastructure Layer - เข้าถึงข้อมูลจากฐานข้อมูล
pub mod setting;       // ⚙️  การตั้งค่าระบบ
pub mod time_helper;   // ⏰  เครื่องมือจัดการเวลา
pub mod usecases;      // 🧠  Application Layer - ตรรกะทางธุรกิจ