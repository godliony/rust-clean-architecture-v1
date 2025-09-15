# 📖 คู่มือการอ่านโค้ด Rust Clean Architecture

## 🎯 ลำดับการอ่านโค้ดที่แนะนำ

### 1️⃣ เริ่มต้น: `src/lib.rs` และ `src/main.rs`
- **`lib.rs`**: ดูโครงสร้างทั้งหมดของโปรเจค
- **`main.rs`**: ดูจุดเริ่มต้นและการประกอบ dependencies

### 2️⃣ Domain Layer: `src/entities/`
- **`entities/items.rs`**: ข้อมูลหลักของระบบ (Business Objects)
- เป็นหัวใจของระบบ ไม่พึ่งพาเทคโนโลยีภายนอก

### 3️⃣ Infrastructure Layer: `src/repositories/`
- **`repositories/items.rs`**: Interface สำหรับการเข้าถึงข้อมูล + Mock generation
- **`repositories/staff.rs`**: Implementation จริงสำหรับ Staff

### 4️⃣ Application Layer: `src/usecases/`
- **`usecases/staff.rs`**: ตรรกะทางธุรกิจ
- **`usecases/staff_test.rs`**: การทดสอบด้วย Mock objects

### 5️⃣ Interface Layer: `src/handlers/`
- **`handlers/staff.rs`**: จัดการ HTTP requests/responses

### 6️⃣ Support Files: `src/models/`, `src/database.rs`, etc.
- **`models/`**: โครงสร้างข้อมูลสำหรับ API
- **`database.rs`**: การเชื่อมต่อฐานข้อมูล
- **`setting.rs`**: การตั้งค่าระบบ

---

## 🏗️ Clean Architecture Layers

```
🌐 Interface Layer (handlers/)
    ↓ calls
🧠 Application Layer (usecases/)
    ↓ calls
🏛️ Domain Layer (entities/)
    ↑ used by
💾 Infrastructure Layer (repositories/)
```

### 📋 Dependency Rules
- **ภายนอก → ภายใน**: Outer layers ขึ้นอยู่กับ inner layers
- **ภายใน ≠ ภายนอก**: Inner layers ไม่รู้จัก outer layers
- **Inversion of Control**: ใช้ interfaces และ dependency injection

---

## 🧩 สาเหตุของการสร้าง Structs, Traits, และ Types

### 📦 Structs
- **`Items`**: Entity หลักที่เก็บข้อมูลของ item ในระบบ
- **`StaffRepository`**: Implementation จริงสำหรับการเข้าถึงข้อมูล Staff
- **`StaffUsecase`**: จัดการ business logic ของ Staff

### 🔧 Traits
- **`ItemsRepository`**:
  - Interface สำหรับการเข้าถึงข้อมูล
  - ทำให้สามารถเปลี่ยน implementation ได้ (Database, Mock, File, etc.)
  - รองรับ Dependency Inversion Principle

### 📝 Types
- **`SharedItemsRepository`**:
  - Type alias สำหรับ `Arc<dyn ItemsRepository + Send + Sync>`
  - ใช้แชร์ Repository ระหว่าง threads ได้อย่างปลอดภัย
  - `Arc` = Atomic Reference Counting สำหรับ shared ownership

---

## 🎭 การทำงานของ Mock Objects

### ❓ Mock คืออะไร?
Mock เป็นการ**จำลองพฤติกรรม**ของ dependencies เพื่อการทดสอบ

### 💡 ทำไมต้องใช้ Mock?
1. **แยกการทดสอบ**: ทดสอบ Use Case โดยไม่ต้องใช้ฐานข้อมูลจริง
2. **ควบคุมผลลัพธ์**: กำหนดได้ว่าต้องการให้ dependency ส่งกลับอะไร
3. **ทดสอบได้เร็ว**: ไม่ต้องรอการเชื่อมต่อฐานข้อมูล
4. **ทดสอบ Error Cases**: จำลองสถานการณ์ผิดพลาดได้ง่าย

### 🛠️ วิธีการทำงาน

#### 1. สร้าง Mock Repository
```rust
let mut items_repository_mock = MockItemsRepository::new();
```

#### 2. กำหนดพฤติกรรม (Setup Expectations)
```rust
items_repository_mock
    .expect_find_by_name()           // คาดหวังให้มีการเรียก
    .with(eq("wooden staff"))        // ด้วยพารามิเตอร์นี้
    .times(1)                        // เรียก 1 ครั้ง
    .returning(|_| Box::pin(async {  // ส่งกลับผลลัพธ์นี้
        Err(sqlx::Error::RowNotFound)
    }));
```

#### 3. ใช้ Mock ในการทดสอบ
```rust
let staff_usecase = StaffUsecase::creation(
    Arc::new(items_repository_mock), // ใช้ Mock แทนของจริง
    timer_helper
);

let result = staff_usecase.adding(req).await; // ทดสอบ
```

### 🔍 จุดประสงค์ของแต่ละ Mock Setup

#### Mock 1: `find_by_name()` → `RowNotFound`
- **จุดประสงค์**: จำลองสถานการณ์ที่ยังไม่มี Staff ชื่อนี้
- **เหตุผล**: เพื่อให้ Use Case ดำเนินการเพิ่มข้อมูลใหม่ได้

#### Mock 2: `insert()` → `Ok(1)`
- **จุดประสงค์**: จำลองการบันทึกสำเร็จและได้ ID = 1
- **เหตุผล**: เพื่อทดสอบว่า Use Case จัดการกับข้อมูลที่บันทึกแล้วอย่างไร

#### Mock 3: `find_by_id(1)` → `Ok(Entity)`
- **จุดประสงค์**: จำลองการดึงข้อมูลที่เพิ่งบันทึกกลับมา
- **เหตุผล**: เพื่อยืนยันว่าข้อมูลถูกบันทึกและสามารถดึงกลับมาได้

---

## 🚀 การใช้งานจริง

### เมื่อรัน Test
```bash
cargo test
```

### เมื่อรัน Application
```bash
cargo run
```
- Server จะเริ่มที่ port ที่กำหนดใน config
- ใช้ฐานข้อมูล PostgreSQL จริง
- Route: `POST /items/staff`

---

## 💡 Tips สำหรับการอ่านโค้ด

1. **เริ่มจาก `main.rs`**: เพื่อเข้าใจ flow ทั้งหมด
2. **ตาม Dependencies**: ดูว่าแต่ละ layer ใช้อะไรจาก layer อื่น
3. **ดู Tests**: เข้าใจพฤติกรรมที่คาดหวังจากโค้ด
4. **ศึกษา Traits**: เข้าใจ interfaces และการทำ abstraction
5. **ติดตาม Error Handling**: ดูว่าระบบจัดการข้อผิดพลาดอย่างไร

---

## 🔧 Tools & Libraries ที่ใช้

- **axum**: Web framework สำหรับ HTTP server
- **sqlx**: Database library สำหรับ PostgreSQL
- **tokio**: Async runtime
- **mockall**: Library สำหรับสร้าง Mock objects
- **chrono**: Date/time handling
- **tracing**: Logging และ observability