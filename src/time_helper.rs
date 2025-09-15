use std::sync::Arc;

use chrono::{NaiveDateTime, TimeZone, Utc};
use mockall::automock;

//dyn IntoTimerHelper เวลาที่เราต้องการ return trait as a type เราต้องประกาศเป็น dyn ... เสมอ 
//เนื่องจาก dyn เป็นตัวบอกให้รู้ว่าส่วนนี้เราจะให้ size ของ trait มัน dynamics ตาม struct ที่ impl ไป

//Arc เปรียบเสมือน Rc แต่ว่าเป็น Rc ที่เป็น atomic นั่นหมายความว่า 
//มันจะอนุญาตให้ตัวแปรตัวอื่นๆหลายๆตัว มา point Arc ได้พร้อมกันในขณะที่อยู่คนละ thread กันก็ตาม

//Arc<dyn IntoTimerHelper + Send + Sync> ถ้าหากเราต้องการที่จะ return trait as a type 
//ในกรณีที่มันเป็น async เราจะต้องมี Send + Sync มา implement ด้วยเสมอ เดี๋ยวจะอธิบายต่อในส่วนของ Send กับ Sync ข้างล่างนะ

//Send เป็น keyword ที่บอก Rust ว่า อะไรก็ตามที่มีการ implement Send ไปนั้น จะทำให้เวลาส่งค่าข้าม thread แล้วมันจะไม่ระเบิด แล้วในกรณีไหนล่ะ ที่มันจะระเบิดได้ ???

//ยกตัวอย่างเช่น เราประกาศ pointer มาตัวนึง แล้วเราก็ทำการ point ไปหาตัวแปรใดๆปกติ แล้วดันทะลึ่งเอาตัวแปรนี้ไปใช้ข้าม thread กัน อย่างเช่น thread:A และ thread: B

//ปรากกฎว่าดันมี thread: Aที่ใช้เสร็จงาน pointer นั้นเสร็จก่อนแล้วทำการ deallocate pointer นั้นทิ้งไป อาจจะทำให้ thread: B ที่ทำงานแบบไม่รู้อิโหน่อิเหน่อยู่ระเบิดได้นั่นเอง

//Sync ก็คือ การอนุญาตให้ตัวแปรนั้นสามารถทำงานพร้อมกันหลายๆ thread ได้ในเวลาเดียวกัน

//ดังนั้นแล้วการที่มี Send และ Sync มักจะมาคู่กันเสมอ เลยเป็นที่มาของ Send + Sync
pub type IntoTimerHelperShared = Arc<dyn IntoTimerHelper + Send + Sync>;

#[automock]
pub trait IntoTimerHelper {
    fn now(&self) -> NaiveDateTime;
}

pub enum  TimerHelper {
    Directly,
    Mock,
}

impl TimerHelper {
    pub fn creation(&self) -> IntoTimerHelperShared  {
        match self{
            Self::Directly => Arc::new(Self::Directly),
            Self::Mock => Arc::new(Self::Mock),
        }
    }
}

impl IntoTimerHelper for TimerHelper {
    fn now(&self) -> NaiveDateTime{
        match self {
            Self::Directly => Utc::now().naive_local(),
            Self::Mock => Utc
                .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                .unwrap()
                .naive_utc(),
        }
    }
}
