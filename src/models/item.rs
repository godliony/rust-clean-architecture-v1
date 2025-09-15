use crate::{entities::items::Items as ItemsEntity, time_helper::IntoTimerHelperShared};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

//Serialize: แปลงข้อมูลจาก struct/enum เป็นรูปแบบอื่น (JSON, XML, YAML เป็นต้น)
//Deserialize: แปลงข้อมูลจากรูปแบบอื่นกลับมาเป็น struct/enum
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Category {
    Staff,
    Sword,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //write!(f, "text") - เขียน string ธรรมดา
            Self::Staff => write!(f, "Staff"),
            Self::Sword => write!(f, "Sword"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub category: Category,
}

impl Item {
    pub fn to_entity(&self, t: IntoTimerHelperShared) -> ItemsEntity {
        ItemsEntity::new(self.name.to_string(), self.category.to_string(), t)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct StaffAdding {
    pub name: String,
}
