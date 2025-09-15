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


หากต้องการเพิ่ม Bow ในระบบ ต้องแก้ไขตามลำดับขั้นตอนนี้:

  ขั้นตอนที่ 1: Models Layer (API)

  📁 src/models/item.rs
  - เพิ่ม Bow ใน Category enum
  - เพิ่ม Display implementation สำหรับ Bow

  #[derive(Serialize, Deserialize, Clone, PartialEq)]
  pub enum Category {
      Staff,
      Sword,
      Bow,    // ← เพิ่มใหม่
  }

  impl Display for Category {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match self {
              Self::Staff => write!(f, "Staff"),
              Self::Sword => write!(f, "Sword"),
              Self::Bow => write!(f, "Bow"),    // ← เพิ่มใหม่
          }
      }
  }

  ขั้นตอนที่ 2: Entities Layer (Domain)

  📁 src/entities/items.rs
  - เพิ่ม "Bow" ใน get_category() method

  pub fn get_category(&self) -> Option<Category>{
      match self.category.as_str(){
          "Staff" => Some(Category::Staff),
          "Sword" => Some(Category::Sword),
          "Bow" => Some(Category::Bow),    // ← เพิ่มใหม่
          _ => None,
      }
  }

  ขั้นตอนที่ 3: Use Cases Layer

  📁 src/usecases/bow.rs - สร้างไฟล์ใหม่
  - สร้าง BowUsecase struct
  - implement adding() method

  ขั้นตอนที่ 4: Repositories Layer

  📁 src/repositories/bow.rs - สร้างไฟล์ใหม่
  - สร้าง BowRepository struct
  - implement ItemsRepository trait
  - เปลี่ยน SQL WHERE condition เป็น category = 'Bow'

  ขั้นตอนที่ 5: Handlers Layer (API)

  📁 src/handlers/bow.rs - สร้างไฟล์ใหม่
  - สร้าง bow_adding() function

  ขั้นตอนที่ 6: Models Layer (เพิ่ม DTO)

  📁 src/models/item.rs
  - เพิ่ม BowAdding struct (ถ้าต้องการ API แยก)

  ขั้นตอนที่ 7: Registration & Routing

  📁 src/lib.rs หรือ src/main.rs
  - เพิ่ม module declarations
  - เพิ่ม route /items/bow
  - register dependencies

  ขั้นตอนที่ 8: Testing

  📁 src/usecases/bow_test.rs - สร้างไฟล์ใหม่
  - เพิ่ม unit tests สำหรับ BowUsecase

  ลำดับการทำงาน:

  1. Models → 2. Entities → 3. Use Cases → 4. Repositories → 5. Handlers → 6. Registration → 7. Testing

  หลักการ: แก้ไขจาก ใน (Domain) ไปยัง นอก (Infrastructure) ตาม Clean Architecture layers


หากต้องการเพิ่ม Character feature (เดิมมีแค่ Weapon) ต้องสร้างระบบใหม่ทั้งหมด:

  ขั้นตอนที่ 1: Database Schema

  📁 Database
  - สร้างตาราง characters ใหม่
  - กำหนด fields: id, name, character_type, created_at, updated_at

  ขั้นตอนที่ 2: Models Layer (API)

  📁 src/models/character.rs - สร้างไฟล์ใหม่
  // เพิ่ม CharacterType enum
  pub enum CharacterType {
      Warrior,
      Mage,
      Archer,
  }

  // เพิ่ม Character model
  pub struct Character {
      pub id: i32,
      pub name: String,
      pub character_type: CharacterType,
  }

  // เพิ่ม CharacterAdding model
  pub struct CharacterAdding {
      pub name: String,
      pub character_type: CharacterType,
  }

  ขั้นตอนที่ 3: Entities Layer (Domain)

  📁 src/entities/characters.rs - สร้างไฟล์ใหม่
  // สร้าง Characters entity
  pub struct Characters {
      pub id: Option<i32>,
      pub name: String,
      pub character_type: String,
      pub created_at: NaiveDateTime,
      pub updated_at: NaiveDateTime,
  }

  // เพิ่ม methods: new(), to_model(), get_character_type()

  ขั้นตอนที่ 4: Repositories Layer

  📁 src/repositories/characters.rs - สร้างไฟล์ใหม่
  // สร้าง CharactersRepository trait
  #[async_trait]
  #[automock]
  pub trait CharactersRepository {
      async fn find_by_name(&self, name: String) -> Result<Characters, sqlx::Error>;
      async fn insert(&self, character: Characters) -> Result<i32, sqlx::Error>;
      async fn find_by_id(&self, id: i32) -> Result<Characters, sqlx::Error>;
  }

  📁 src/repositories/warrior.rs - สร้างไฟล์ใหม่
  // สร้าง WarriorRepository struct
  // implement CharactersRepository trait
  // SQL queries สำหรับตาราง characters

  ขั้นตอนที่ 5: Use Cases Layer

  📁 src/usecases/warrior.rs - สร้างไฟล์ใหม่
  // สร้าง WarriorUsecase struct
  pub struct WarriorUsecase {
      characters_repository: SharedCharactersRepository,
      timer_helper: IntoTimerHelperShared,
  }

  // เพิ่ม adding() method

  ขั้นตอนที่ 6: Handlers Layer (API)

  📁 src/handlers/warrior.rs - สร้างไฟล์ใหม่
  // สร้าง warrior_adding() function
  pub async fn warrior_adding(
      Json(body): Json<CharacterAdding>,
      warrior_usecase: Arc<WarriorUsecase>,
  ) -> impl IntoResponse {
      // logic สำหรับ HTTP request/response
  }

  ขั้นตอนที่ 7: Error Handling

  📁 src/models/error.rs
  - เพิ่ม Character-related errors:
    - CharacterAlreadyExists
    - CharacterNotFound
    - AddingCharacterError

  ขั้นตอนที่ 8: Registration & Routing

  📁 src/lib.rs
  // เพิ่ม module declarations
  pub mod entities {
      pub mod characters;
  }
  pub mod repositories {
      pub mod characters;
      pub mod warrior;
  }
  pub mod usecases {
      pub mod warrior;
  }
  pub mod handlers {
      pub mod warrior;
  }

  📁 src/main.rs
  - เพิ่ม route /characters/warrior
  - register WarriorUsecase และ WarriorRepository

  ขั้นตอนที่ 9: Testing

  📁 src/usecases/warrior_test.rs - สร้างไฟล์ใหม่
  - Unit tests สำหรับ WarriorUsecase

  สรุปไฟล์ที่ต้องสร้างใหม่:

  1. src/models/character.rs
  2. src/entities/characters.rs
  3. src/repositories/characters.rs (interface)
  4. src/repositories/warrior.rs (implementation)
  5. src/usecases/warrior.rs
  6. src/handlers/warrior.rs
  7. src/usecases/warrior_test.rs

  ไฟล์ที่ต้องแก้ไข:

  1. src/models/error.rs - เพิ่ม error types
  2. src/lib.rs - เพิ่ม module declarations
  3. src/main.rs - เพิ่ม routing และ dependency injection

  ลำดับ: Database → Models → Entities → Repositories → Use Cases → Handlers → Registration → Testing