use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub challenge_id: String,
    pub toc: SystemTime,
}

