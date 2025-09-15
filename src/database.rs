use crate::setting::Setting;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// นำเข้า Arc (Atomic Reference Counted) จาก standard library
// Arc เป็นตัว smart pointer ที่ใช้แชร์ข้อมูลระหว่าง threads ได้อย่างปลอดภัย
use std::sync::Arc;

pub async fn conn_getting(setting: Arc<Setting>) -> Result<Pool<Postgres>,sqlx::Error>{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(setting.database.url_getting().as_str())
        .await?;
    Ok(pool)
}