// === 🧪 Test Module: Staff Use Case Tests ===
// 📖 การทดสอบ Use Case โดยใช้ Mock objects เพื่อแยกการทดสอบออกจากฐานข้อมูลจริง

#[cfg(test)]           // 🔧 compile เฉพาะตอน run tests เท่านั้น
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;    // 🎭 Mock library สำหรับจำลองพฤติกรรม

    use crate::{
        entities::items::Items as ItemsEntity,      // 🏛️ Domain Entity
        models::item::{Category, StaffAdding},      // 📋 API Models
        repositories::items::MockItemsRepository,   // 🎭 Mock Repository (จำลอง)
        time_helper::TimerHelper,                   // ⏰ Timer utility
        usecases::staff::StaffUsecase,              // 🧠 Use Case ที่จะทดสอบ
    };

    // 🧪 Test Function: ทดสอบการเพิ่ม Staff ใหม่
    #[tokio::test]         // 🔧 async test attribute สำหรับ tokio runtime
    async fn adding_test(){
        // 🎭 สร้าง Mock Repository - จำลองพฤติกรรมของ Repository โดยไม่ต้องใช้ฐานข้อมูลจริง
        let mut items_repository_mock = MockItemsRepository::new();

        // ⏰ สร้าง Mock Timer Helper - ใช้เวลาที่กำหนดไว้ล่วงหน้าแทนที่เวลาปัจจุบัน
        let timer_helper = TimerHelper::Mock.creation();

        // 📥 เตรียมข้อมูล input สำหรับการทดสอบ

        // 🔄 แบบเดิม: ไม่มี category field
        // let req = StaffAdding {
        //     name: "wooden staff".to_string(),
        // };

        // ✅ แบบใหม่: เพิ่ม category field ตาม Clean Architecture pattern
        let req = StaffAdding {
            name: "wooden staff".to_string(),
            category: Category::Staff,
        };

        // 🎭 Mock Setup 1: กำหนดพฤติกรรมของ find_by_name()
        // จำลองสถานการณ์ที่ไม่พบ Staff ชื่อนี้ในระบบ (เพื่อให้เพิ่มได้)
        items_repository_mock
            .expect_find_by_name()                      // 🔍 คาดหวังให้มีการเรียก find_by_name
            .with(eq(req.name.clone()))                 // 📋 ด้วยพารามิเตอร์ที่ตรงกับชื่อที่ต้องการเพิ่ม
            .times(1)                                   // 🔢 เรียกครั้งเดียวเท่านั้น
            .returning(|_| Box::pin(async {Err(sqlx::Error::RowNotFound)})); // ❌ ส่งกลับ "ไม่พบข้อมูล"

        // 🎭 Mock Setup 2: กำหนดพฤติกรรมของ insert()
        // จำลองการบันทึกข้อมูลใหม่สำเร็จและได้ ID = 1 กลับมา
        items_repository_mock
            .expect_insert()                            // ➕ คาดหวังให้มีการเรียก insert
            .with(eq(ItemsEntity::new(                  // 📋 ด้วย Entity ที่มีข้อมูลตรงตามที่คาดหวัง
                req.name.clone(),
                Category::Staff.to_string(),
                Arc::clone(&timer_helper),
            )))
            .returning(|_| Box::pin(async {Ok(1)}));    // ✅ ส่งกลับ ID = 1 (สำเร็จ)

        // 🎭 Mock Setup 3: กำหนดพฤติกรรมของ find_by_id()
        // จำลองการดึงข้อมูลที่เพิ่งบันทึกกลับมาเพื่อยืนยัน
        items_repository_mock
            .expect_find_by_id()                        // 🔍 คาดหวังให้มีการเรียก find_by_id
            .with(eq(1))                                // 📋 ด้วย ID = 1
            .times(1)                                   // 🔢 เรียกครั้งเดียวเท่านั้น
            .returning(|_| {                            // 🔄 ส่งกลับข้อมูล Entity ที่สมบูรณ์
                Box::pin(async{
                    let t = TimerHelper::Mock.creation();
                    Ok(ItemsEntity {
                        id: Some(1),                    // 🆔 มี ID แล้ว
                        name: "wooden staff".to_string(),
                        category: Category::Staff.to_string(),
                        created_at: t.now(),
                        updated_at: t.now(),
                    })
                })
            });

        // 🏗️ สร้าง Use Case พร้อม Mock dependencies
        let staff_usecase = StaffUsecase::creation(
            Arc::new(items_repository_mock),            // 🎭 ใช้ Mock Repository แทนของจริง
            timer_helper                                // ⏰ ใช้ Mock Timer
        );

        // 🧪 ทดสอบการเรียกใช้งาน Use Case
        let result = match staff_usecase.adding(req).await {
            Ok(r) => r,                                 // ✅ คาดหวังให้สำเร็จ
            Err(_) => panic!("adding error"),           // ❌ ถ้าผิดพลาดให้ panic
        };

        // 🔍 ตรวจสอบว่าได้ ID กลับมา
        let id = match Some(result.id){
            Some(i) => i,                               // ✅ มี ID
            None => panic!("id is None"),               // ❌ ไม่มี ID (ไม่ควรเกิดขึ้น)
        };

        // ✅ Assertions: ตรวจสอบผลลัพธ์
        assert_eq!(result.id, id);                      // 🆔 ID ต้องตรงกัน
        assert_eq!(result.name, "wooden staff");       // 📝 ชื่อต้องตรงกับที่ส่งเข้ามา

    } // 🎉 การทดสอบเสร็จสิ้น - ทำให้มั่นใจว่า Use Case ทำงานถูกต้อง
}
