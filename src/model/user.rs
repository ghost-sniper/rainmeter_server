use chrono::{Date, DateTime, Local};
use chrono::prelude::*;
use mysql::{serde, Statement};
use mysql::serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::get_connection;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoreUser {
    user_id: u64,
    username: Option<String>,
    age: u8,
    phone: Option<String>,
    email: Option<String>,
    birthday: Option<Date<Local>>,
    created_by: Option<String>,
    created_time: Option<DateTime<Local>>,
    updated_by: Option<String>,
    updated_time: Option<DateTime<Local>>,
}

pub fn get_user(user_id: u64) -> CoreUser {
    let conn = get_connection();
    let state = Statement::from();
}

pub fn insert_user(user: CoreUser) {
    let conn = get_connection();
    let state = Statement::from();
}