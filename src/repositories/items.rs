// นำเข้า Arc (Atomic Reference Counted) จาก standard library
// Arc เป็นตัว smart pointer ที่ใช้แชร์ข้อมูลระหว่าง threads ได้อย่างปลอดภัย
use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;

use crate::entities::items::Items;

pub type SharedItemsRepository = Arc<dyn ItemsRepository + Send + Sync>;

//#[async_trait] เราจะเอาไว้ใช้ในกรณีที methods ต่างๆของเราใน trait มันเป็น async
//#[automock] อันนี้เอาไว้ทำ mock อัตโนมัติจาก trait จะได้ไม่ต้องมานั่งเขียนเองให้ปวดหลัง
#[async_trait]
#[automock]
pub trait ItemsRepository {
    async fn find_by_name(&self, name: String) -> Result<Items, sqlx::Error>;
    async fn insert(&self, item: Items) -> Result<i32, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Items, sqlx::Error>;
}
